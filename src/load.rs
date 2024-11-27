use crate::{
    crossword::CrosswordData,
    utils::{calculate_checksum, extract_cstring, extract_cstring_with_offset},
    Footer, Game, GameData, Header, Metadata, TggFile,
};

pub fn load(bytes: Vec<u8>) -> Result<TggFile, String> {
    // Validate and extract header
    if bytes.len() < 24 {
        return Err("Failed to load file: insufficient data".to_string());
    }

    let header_bytes = &bytes[0..22];
    if header_bytes.len() != 22 {
        return Err("Failed to load file: invalid header length".to_string());
    }

    let version = extract_cstring(&header_bytes[0..5]);
    let id = extract_cstring(&header_bytes[5..19]);

    if id != "TalonGamesGame" {
        return Err("Failed to load file: invalid ID".to_string());
    }

    let game = Game::from_byte(header_bytes[19]).ok_or("Failed to load file: invalid game type")?;
    let file_checksum = u16::from_le_bytes([header_bytes[20], header_bytes[21]]);

    // Validate body length
    if bytes.len() < 24 {
        return Err("Failed to load file: insufficient body data".to_string());
    }

    let body = &bytes[22..bytes.len() - 2];

    // Extract metadata
    let (title, offset) = extract_cstring_with_offset(body, 0);
    let (description, offset) = extract_cstring_with_offset(body, offset);
    let (author, offset) = extract_cstring_with_offset(body, offset);

    if offset + 6 > body.len() {
        return Err("Failed to load file: incomplete metadata".to_string());
    }

    let creation_date_bytes = &body[offset..offset + 4];
    let creation_date = u32::from_be_bytes(creation_date_bytes.try_into().unwrap());

    let checksum_bytes = &body[offset + 4..offset + 6];
    let gamedata_checksum = u16::from_le_bytes(checksum_bytes.try_into().unwrap());

    // Verify checksums
    let calculated_checksum = u16::from_le_bytes(calculate_checksum(body.to_vec()));
    let footer_file_checksum = u16::from_le_bytes([bytes[bytes.len() - 2], bytes[bytes.len() - 1]]);
    if file_checksum != calculated_checksum || file_checksum != footer_file_checksum {
        return Err("Failed to load file: checksum mismatch".to_string());
    }

    let game_data = &body[offset + 6..body.len()];

    let gamedata: GameData = match game {
        Game::Crossword => {
            let crossword_data = match CrosswordData::load(game_data) {
                Ok(crossword_data) => crossword_data,
                Err(err) => return Err(err),
            };

            GameData::Crossword(crossword_data)
        }
        Game::WordSearch => !unimplemented!("no done"),
    };

    let header = Header::new(version, game, file_checksum);
    let metadata = Metadata::new(title, description, author, creation_date, gamedata_checksum);
    let footer = Footer::new(file_checksum);

    Ok(TggFile {
        header,
        metadata,
        gamedata,
        footer,
    })
}
