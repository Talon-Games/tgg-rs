use super::utils::calculate_checksum;
use super::Game;

pub fn load(bytes: Vec<u8>) -> Result<(), &'static str> {
    let header_bytes = &bytes[0..22];
    println!("{:02X?}", header_bytes);
    if header_bytes.len() != 22 {
        return Err("Failed to load file: invalid header length");
    }
    let version = header_bytes[0..5]
        .iter()
        .filter(|byte| **byte != 0x00)
        .map(|byte| *byte as char)
        .collect::<String>();
    let id = header_bytes[5..19]
        .iter()
        .filter(|byte| **byte != 0x00)
        .map(|byte| *byte as char)
        .collect::<String>();

    let game = match Game::from_byte(header_bytes[19]) {
        Some(game) => game,
        None => return Err("Failed to load file: invalid game type"),
    };

    let file_checksum = u16::from_le_bytes([header_bytes[20], header_bytes[21]]);

    println!("{}", version);
    println!("{}", id);
    println!("{}", game.to_string());
    println!("sum: {}", file_checksum);

    let body = &bytes[22..bytes.len() - 2];
    let footer_file_checksum = &[bytes[bytes.len() - 2], bytes[bytes.len() - 1]];
    if file_checksum != u16::from_le_bytes(calculate_checksum(body.to_vec()))
        || file_checksum != u16::from_le_bytes(*footer_file_checksum)
    {
        return Err("Failed to load file: file checksum mismatch, file may be corrupted");
    }

    Ok(())
}
