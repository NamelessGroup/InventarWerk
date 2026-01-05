use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema)]
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
    pub school: Vec<String>,
    pub range: Vec<String>,
    pub class: Vec<String>,
    pub sub_class: Vec<String>,
    pub species: Vec<String>,
    pub feats: Vec<String>
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpellList {
    pub uuid: String,
    pub name: String,
    pub owner_uuid: String,
    pub spell_uuids: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpellSlot {
    pub owner_uuid: String,
    pub spell_slot_level: i32,
    pub spell_slot_maxima: i32,
    pub spell_slot_available: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Concentration {
    pub owner_uuid: String,
    pub concentration: i32,
}
