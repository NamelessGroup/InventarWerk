
//uuid
#[get("/spelllist")]
pub async fn get_spell_list() {

}

//name
#[put("/spelllist")]
pub async fn create_spelllist() {

}


#[get("/spelllist/all")]
pub async fn get_all_spell_list() {

}

//uuid
#[delete("/spelllist/delete")]
pub async fn delete_spell_list() {

}
// uuid name
#[patch("/spelllist/edit")]
pub async fn patch_spell_list() {

}
// uuid presetid
#[put("/spelllist/spell/addPreset")]
pub async fn add_preset_to_list() {

}

// uuid,presetuuid
#[delete("/spelllist/spell/remove")]
pub async fn delete_preset_from_list() {
    
}
