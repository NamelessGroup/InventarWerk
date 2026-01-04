use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
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
    uuid: String,
    name: String,
    owner_uuid: String,
    spell_uuids: Vec<String>,
}
