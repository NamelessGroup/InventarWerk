use rocket::Route;
use utoipa::IntoParams;

/// Documentation of the optional `owner_uuid` query parameter that is evaluated by the
/// `utils::TargetUser` guard. Only used for the OpenAPI docs, not for parsing.
#[derive(IntoParams)]
#[into_params(parameter_in = Query)]
pub struct OwnerUUID {
    /// UUID of the user to act on. If omitted or equal to the requesting user, the requesting user is used. Acting on another user requires DM privileges.
    #[allow(dead_code)]
    owner_uuid: Option<String>,
}

pub mod concentration_router;
pub mod spell_attack_modifier_router;
pub mod spell_cast_router;
pub mod spell_list_router;
pub mod spell_preset_router;
pub mod spell_slot_router;

use concentration_router::*;
use spell_attack_modifier_router::*;
use spell_cast_router::*;
use spell_list_router::*;
use spell_preset_router::*;
use spell_slot_router::*;


pub fn get_concentration_routes() -> Vec<Route> {
    routes![
        get_concentration,
        get_all_concentrations,
        remove_concentration
    ]
}

pub fn get_spell_attack_modifier_routes() -> Vec<Route> {
    routes![
        get_spell_attack_modifier,
        get_all_spell_attack_modifiers,
        set_spell_attack_modifier
    ]
}

pub fn get_spell_list_routes() -> Vec<Route> {
    routes![
        get_spell_list,
        get_all_spell_list,
        create_spelllist,
        delete_spell_list,
        patch_spell_list,
        add_preset_to_list
    ]
}

pub fn get_spell_preset_routes() -> Vec<Route> {
    routes![
        get_spell_preset,
        get_all_spell_presets,
        delete_spell_preset,
        edit_spell_preset,
        add_extern_spell_preset,
        delete_preset_from_list
    ]
}

pub fn get_spell_cast_routes() -> Vec<Route> {
    routes![
        cast_spell,
        spell_cast_stream
    ]
}

pub fn get_spell_slot_routes() -> Vec<Route> {
    routes![
        get_spell_slots,
        patch_available_spell_slots,
        patch_maxima_spell_slots,
        reset_spell_slots
    ]
}