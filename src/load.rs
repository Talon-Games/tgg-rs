use crate::{utils::calculate_checksum, Footer, Game, Header, Metadata};

pub fn load(bytes: Vec<u8>) -> Result<(), &'static str> {
    // Validate and extract header
    if bytes.len() < 24 {
        return Err("Failed to load file: insufficient data");
    }

    let header_bytes = &bytes[0..22];
    if header_bytes.len() != 22 {
        return Err("Failed to load file: invalid header length");
    }

    let version = extract_cstring(&header_bytes[0..5]);
    let id = extract_cstring(&header_bytes[5..19]);

    if id != "TalonGamesGame" {
        return Err("Failed to load file: invalid ID");
    }

    let game = Game::from_byte(header_bytes[19]).ok_or("Failed to load file: invalid game type")?;
    let file_checksum = u16::from_le_bytes([header_bytes[20], header_bytes[21]]);

    println!("{version}");
    println!("{id}");
    println!("{}", game.to_string());
    println!("Checksum: {file_checksum}");

    // Validate body length
    if bytes.len() < 24 {
        return Err("Failed to load file: insufficient body data");
    }

    let body = &bytes[22..bytes.len() - 2];

    // Extract metadata
    let (title, offset) = extract_cstring_with_offset(body, 0);
    let (description, offset) = extract_cstring_with_offset(body, offset);
    let (author, offset) = extract_cstring_with_offset(body, offset);

    println!("{title}");
    println!("{description}");
    println!("{author}");

    if offset + 6 > body.len() {
        return Err("Failed to load file: incomplete metadata");
    }

    let creation_date_bytes = &body[offset..offset + 4];
    let creation_date = u32::from_be_bytes(creation_date_bytes.try_into().unwrap());

    let checksum_bytes = &body[offset + 4..offset + 6];
    let gamedata_checksum = u16::from_le_bytes(checksum_bytes.try_into().unwrap());

    // Verify checksums
    let calculated_checksum = u16::from_le_bytes(calculate_checksum(body.to_vec()));
    let footer_file_checksum = u16::from_le_bytes([bytes[bytes.len() - 2], bytes[bytes.len() - 1]]);
    if file_checksum != calculated_checksum || file_checksum != footer_file_checksum {
        return Err("Failed to load file: checksum mismatch, file may be corrupted");
    }

    println!("Creation date: {creation_date}");
    println!("Game data checksum: {}", &gamedata_checksum);

    let game_data = &body[offset + 6..body.len()];

    let header = Header::new(game, file_checksum);
    let metadata = Metadata::new(title, description, author, creation_date, gamedata_checksum);
    let footer = Footer::new(file_checksum);

    Ok(())
}

fn extract_cstring(bytes: &[u8]) -> String {
    bytes
        .iter()
        .take_while(|&&byte| byte != 0x00)
        .map(|&byte| byte as char)
        .collect()
}

fn extract_cstring_with_offset(bytes: &[u8], start: usize) -> (String, usize) {
    let mut end = start;
    while end < bytes.len() && bytes[end] != 0x00 {
        end += 1;
    }
    let result = bytes[start..end]
        .iter()
        .map(|&byte| byte as char)
        .collect::<String>();
    (result, end + 1) // +1 to skip the null terminator
}