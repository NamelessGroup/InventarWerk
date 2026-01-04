use utils::AuthenticatedUser;
//uuid
#[get("/spelllist")]
pub async fn get_spell_list(_user: AuthenticatedUser) {

}

//name
#[put("/spelllist")]
pub async fn create_spelllist(_user: AuthenticatedUser) {

}


#[get("/spelllist/all")]
pub async fn get_all_spell_list(_user: AuthenticatedUser) {

}

//uuid
#[delete("/spelllist/delete")]
pub async fn delete_spell_list(_user: AuthenticatedUser) {

}
// uuid name
#[patch("/spelllist/edit")]
pub async fn patch_spell_list(_user: AuthenticatedUser) {

}
// uuid presetid
#[put("/spelllist/spell/addPreset")]
pub async fn add_preset_to_list(_user: AuthenticatedUser) {

}

// uuid,presetuuid
#[delete("/spelllist/spell/remove")]
pub async fn delete_preset_from_list(_user: AuthenticatedUser) {
    
}
