use utils::AuthenticatedUser;

//optional id

#[derive(FromForm)]
struct ConcentrationUUID{
    uuid: Option<String>
}

#[get("/concentration?<params..>")]
pub async fn get_concentration(_user: AuthenticatedUser, params: ConcentrationUUID) {
    
}

#[get("/concentration/all")]
pub async fn get_all_concentrations(_user: AuthenticatedUser) {

}
#[put("/concentration")]
pub async fn modify_concentration(_user: AuthenticatedUser) {

}
