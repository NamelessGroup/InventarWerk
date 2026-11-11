use repos::{
    model_zauberwerk::SpellPreset,
    repos::{
        user_repository::UserRepository,
        zauberwek::{
            spell_cast_repository::{CastOutcome, SpellCastRepository},
            spell_list_repository::SpellListRepository,
            spell_preset_repository::SpellPresetRepository,
        },
    },
};
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::{http::Status, serde::json::Json, Shutdown, State};
use rocket_errors::anyhow::Result;
use serde::{Deserialize, Serialize};
use utils::{AuthenticatedUser, TargetUser};

use super::OwnerUUID;
use utoipa::{IntoParams, OpenApi, ToSchema};

/// Highest spell slot level.
const MAX_SPELL_LEVEL: i32 = 9;

/// Event that is broadcast to all clients of `/spellPreset/cast/stream` whenever a spell is cast.
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpellCastEvent {
    /// UUID of the user who cast the spell
    pub caster_uuid: String,
    /// Display name of the user who cast the spell
    pub caster_name: String,
    /// UUID of the user who triggered the cast. Differs from `casterUuid` if a DM cast on behalf of a player.
    pub triggered_by_uuid: String,
    /// The complete spell preset that was cast
    pub spell: SpellPreset,
    /// Level the spell was cast at (0 for cantrips)
    pub cast_level: i32,
    /// Whether the spell was cast at a higher level than its base level
    pub upcast: bool,
    /// Whether a spell slot was consumed (false for cantrips)
    pub slot_consumed: bool,
    /// Available spell slots of `castLevel` after casting, `null` for cantrips
    pub remaining_slots: Option<i32>,
    /// Whether the caster is now concentrating on this spell
    pub concentration: bool,
    /// UUID of the spell preset the caster was concentrating on before and which was replaced by this cast
    pub replaced_concentration_uuid: Option<String>,
    /// Unix timestamp of the cast in milliseconds
    pub timestamp: u128,
}

#[derive(FromForm, ToSchema, IntoParams)]
pub struct CastSpellParams {
    /// UUID of the spell preset to cast
    spell_preset_uuid: String,
    /// Level to cast the spell at. Defaults to the base level of the spell. Must be between the base level and 9. Cantrips can only be cast at level 0.
    level: Option<i32>,
}

#[utoipa::path(
    put,
    path = "/spellPreset/cast",
    summary = "Cast a spell",
    description = r#"Casts a spell for the requesting user (or, for DMs, for the user given by `owner_uuid`).

The spell must be contained in at least one spell list of the caster. In a single transaction:
- one spell slot of the cast level is consumed (cantrips consume no slot),
- if the spell requires concentration, the caster's concentration is set to this spell, replacing any previous concentration.

On success a `SpellCastEvent` with all information about the cast is returned and broadcast to all clients of `GET /spellPreset/cast/stream`. The changes to spell slots and concentration are additionally reported via the regular last-changes stream."#,
    params(OwnerUUID, CastSpellParams),
    responses(
        (status = 200, description = "Spell was cast", body = SpellCastEvent),
        (status = 400, description = "Invalid cast level"),
        (status = 403, description = "Forbidden - foreign owner_uuid given but user is not a DM, or the spell is not in a spell list of the caster"),
        (status = 409, description = "No spell slot of the cast level available"),
        (status = 500, description = "Spell preset not found or database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Casting"
)]
#[put("/spellPreset/cast?<params..>")]
pub async fn cast_spell(
    target: TargetUser,
    params: CastSpellParams,
    usr_rep: &State<UserRepository>,
    spp_rep: &State<SpellPresetRepository>,
    sli_rep: &State<SpellListRepository>,
    cast_rep: &State<SpellCastRepository>,
    queue: &State<Sender<SpellCastEvent>>,
) -> Result<Result<Json<SpellCastEvent>, Status>> {
    let TargetUser { requester_uuid, owner_uuid: caster_uuid } = target;

    let spell = spp_rep.get_spell_preset(&params.spell_preset_uuid).await?;

    let cast_level = params.level.unwrap_or(spell.level);
    let level_valid = if spell.level == 0 {
        cast_level == 0
    } else {
        (spell.level..=MAX_SPELL_LEVEL).contains(&cast_level)
    };
    if !level_valid {
        return Ok(Err(Status::BadRequest));
    }

    if !sli_rep.owner_has_spell(&caster_uuid, &spell.uuid).await? {
        return Ok(Err(Status::Forbidden));
    }

    let caster = usr_rep.get_user(&caster_uuid).await?;

    let (remaining_slots, replaced_concentration_uuid) = match cast_rep
        .cast_spell(&caster_uuid, &spell.uuid, cast_level, spell.concentration)
        .await?
    {
        CastOutcome::Cast { remaining_slots, previous_concentration } => {
            (remaining_slots, previous_concentration)
        }
        CastOutcome::NoSlotAvailable => return Ok(Err(Status::Conflict)),
    };

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    let event = SpellCastEvent {
        caster_uuid,
        caster_name: caster.name,
        triggered_by_uuid: requester_uuid,
        upcast: cast_level > spell.level,
        concentration: spell.concentration,
        spell,
        cast_level,
        slot_consumed: remaining_slots.is_some(),
        remaining_slots,
        replaced_concentration_uuid,
        timestamp,
    };

    // Sending only fails if no client is subscribed, which is fine.
    let _ = queue.send(event.clone());

    Ok(Ok(Json(event)))
}

#[utoipa::path(
    get,
    path = "/spellPreset/cast/stream",
    summary = "Subscribe to spell cast events",
    description = "Opens a Server-Sent Events (SSE) stream that pushes a `SpellCastEvent` (JSON) to every connected client whenever a spell is cast via `PUT /spellPreset/cast`.",
    responses(
        (status = 200, description = "Event stream established", content_type = "text/event-stream", body = SpellCastEvent)
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Casting"
)]
#[get("/spellPreset/cast/stream")]
pub async fn spell_cast_stream(
    _user: AuthenticatedUser,
    queue: &State<Sender<SpellCastEvent>>,
    mut end: Shutdown,
) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        cast_spell,
        spell_cast_stream
    ),
    components(
        schemas(
            SpellCastEvent,
            CastSpellParams,
            SpellPreset
        )
    ),
    tags(
        (name = "Spell Casting", description = "Casting spells and subscribing to cast events")
    )
)]
pub struct SpellCastApiDoc;
