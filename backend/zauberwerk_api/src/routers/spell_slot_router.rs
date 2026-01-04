use utils::AuthenticatedUser;
// optional uuid
#[get("/spellslot")]
pub async fn get_spell_slots(_user: AuthenticatedUser) {

}
// (optional) uuid, 1-9
#[patch("/spellslot/available")]
pub async fn patch_available_spell_slots(_user: AuthenticatedUser) {

}
// (optional) uuid, 1-9
#[patch("/spellslot/maxima")]
pub async fn patch_maxima_spell_slots(_user: AuthenticatedUser) {

}
// (optionl) uuid
#[patch("/spellslot/reset")]
pub async fn reset_spell_slots(_user: AuthenticatedUser) {

}

   
