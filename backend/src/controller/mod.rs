pub mod inventory_controller;
pub mod account_controller;
pub mod item_preset_controller;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rocket::http::Status;
use rocket::response::status::Custom;

//custom status
pub type CStat = Custom<String>;
pub fn new_cstat(stat: Status, msg: String) -> CStat {
    Custom(
        stat,
        msg
    )
}

pub fn new_cstat_from_ref(stat: Status, msg: &'static str) -> CStat {
    Custom(
        stat,
        msg.to_string()
    )
}

pub fn format_result_to_cstat<T>(result: Result<T, diesel::result::Error>, stat: Status, err_msg: &'static str) -> Result<T, CStat> {
    match result {
        Ok(res) => Ok(res),
        Err(e) => Err(new_cstat(stat, format!("{}, more precise: {}", err_msg, e.to_string())))
    }
}

pub fn generate_uuid_v4() -> String {
    let mut rng = StdRng::from_entropy();

    // Generiere 16 zufällige Bytes
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);

    // Setze die UUID-Version (4) und das Variant-Bit (10xx)
    bytes[6] = (bytes[6] & 0x0F) | 0x40; // Setze die 4 im 7. Byte
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Setze die obersten Bits auf 10xx

    // Formatiere die Bytes als UUID-String
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}