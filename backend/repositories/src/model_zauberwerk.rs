use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpellPreset {
    pub uuid: String,
    pub name: String,
    pub level: i32,
    pub description: String,
    pub time: String,
    pub components: Vec<String>,
    pub concentration: bool,
    pub duration: String,
    pub school: String,
    pub range: String,
    pub class: Vec<String>,
    pub sub_class: Vec<String>,
    pub species: Vec<String>,
    pub feats: Vec<String>,
    pub condition_inflict: String,
    pub saving_throw: String,
    pub affects_creature_type: String,
    pub damage_inflict: String,
    pub spell_attack: String,
    pub ritual: bool,
    pub damage_immune: String,
    pub ability_check: String,
    pub damage_resist: String,
    pub condition_immune: String,
    pub higher_level: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpellList {
    pub uuid: String,
    pub name: String,
    pub owner_uuid: String,
    pub spell_uuids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpellSlot {
    pub owner_uuid: String,
    pub spell_slot_level: i32,
    pub spell_slot_maxima: i32,
    pub spell_slot_available: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Concentration {
    pub owner_uuid: String,
    /// UUID of the spell preset the user is concentrating on, or `null` if the user is not concentrating.
    pub spell_preset_uuid: Option<String>,
}
