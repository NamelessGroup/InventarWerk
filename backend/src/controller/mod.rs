pub mod inventory_controller;
pub mod account_controller;

pub fn formatResultToCustomErr<T>(result: Result<T, diesel::result::Error>, err_msg: &'static str) -> Result<T, &'static str> {
    match result {
        Ok(res) => Ok(res),
        Err(_e) => Err(err_msg)
    }
}