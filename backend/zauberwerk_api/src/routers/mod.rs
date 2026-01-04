use rocket::Route;

mod concentration_router;
mod spell_list_router;
mod spell_preset_router;
mod spell_slot_router;

use concentration_router::*;
use spell_list_router::*;
use spell_preset_router::*;
use spell_slot_router::*;


pub fn get_concentration_routes() -> Vec<Route> {
    routes![
        get_concentration,
        get_all_concentrations,
        modify_concentration
    ]
}

pub fn get_spell_list_routes() -> Vec<Route> {
    routes![
        get_spell_list,
        get_all_spell_list,
        create_spelllist,
        delete_spell_list,
        patch_spell_list,
        add_preset_to_list,
        delete_spell_list
    ]
}

pub fn get_spell_preset_routes() -> Vec<Route> {
    routes![
        get_spell_preset,
        get_all_spell_presets,
        delete_spell_preset,
        edit_spell_preset,
        delete_spell_list,
        cast_spell,
        add_extern_spell_preset,
        delete_preset_from_list
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