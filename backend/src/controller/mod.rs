pub mod inventory_controller;
pub mod account_controller;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub fn format_result_to_custom_err<T>(result: Result<T, diesel::result::Error>, err_msg: &'static str) -> Result<T, &'static str> {
    match result {
        Ok(res) => Ok(res),
        Err(_e) => Err(err_msg)
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