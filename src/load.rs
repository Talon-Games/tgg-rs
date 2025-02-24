use crate::{
    crossword::CrosswordData,
    utils::{calculate_checksum, extract_cstring_with_offset},
    Footer, Game, GameData, Header, Metadata, TggFile,
};

pub fn load(bytes: Vec<u8>) -> Result<TggFile, String> {
    // Validate and extract header
    if bytes.len() < 17 {
        return Err(format!(
            "Failed to load file: insufficient data (expected at least 17 bytes, got {})",
            bytes.len()
        ));
    }

    let header_bytes = &bytes[0..17];

    let id: String = header_bytes[0..14]
        .iter()
        .map(|&byte| byte as char)
        .collect();

    if id != "TalonGamesGame" {
        return Err(format!(
            "Failed to load file: invalid ID (expected 'TalonGamesGame', got '{}')",
            id
        ));
    }

    let game = match Game::from_byte(header_bytes[14]) {
        Some(game) => game,
        None => {
            return Err(format!(
                "Failed to load file: invalid game type byte (got 0x{:02X})",
                header_bytes[14]
            ));
        }
    };
    let file_checksum = u16::from_le_bytes([header_bytes[15], header_bytes[16]]);

    let body = &bytes[17..bytes.len() - 2];

    // Extract metadata
    let (title, offset) = extract_cstring_with_offset(body, 0);
    let (description, offset) = extract_cstring_with_offset(body, offset);
    let (author, offset) = extract_cstring_with_offset(body, offset);

    if title.is_empty() {
        return Err("Failed to load file: title is empty".to_string());
    }

    if description.is_empty() {
        return Err("Failed to load file: description is empty".to_string());
    }

    if author.is_empty() {
        return Err("Failed to load file: author is empty".to_string());
    }

    if offset + 6 > body.len() {
        return Err("Failed to load file: incomplete metadata".to_string());
    }

    // Validate metadata boundaries
    if offset + 6 > body.len() {
        return Err(format!(
            "Failed to load file: incomplete metadata (expected at least {} bytes, got {})",
            offset + 6,
            body.len()
        ));
    }

    let creation_date_bytes = &body[offset..offset + 4];
    let creation_date = u32::from_be_bytes(creation_date_bytes.try_into().unwrap());

    let checksum_bytes = &body[offset + 4..offset + 6];
    let gamedata_checksum = u16::from_le_bytes(checksum_bytes.try_into().unwrap());

    // Verify checksums
    let calculated_checksum = u16::from_le_bytes(calculate_checksum(body.to_vec()));
    let footer_file_checksum = u16::from_le_bytes([bytes[bytes.len() - 2], bytes[bytes.len() - 1]]);
    if file_checksum != calculated_checksum {
        return Err(format!(
            "Failed to load file: header checksum mismatch (expected {}, calculated {})",
            file_checksum, calculated_checksum
        ));
    }
    if file_checksum != footer_file_checksum {
        return Err(format!(
            "Failed to load file: footer checksum mismatch (expected {}, found {})",
            file_checksum, footer_file_checksum
        ));
    }

    let game_data = &body[offset + 6..body.len()];
    if game_data.is_empty() {
        return Err("Failed to load file: game data is empty".to_string());
    }

    let gamedata: GameData = match game {
        Game::Crossword => {
            let crossword_data = match CrosswordData::load(game_data) {
                Ok(crossword_data) => crossword_data,
                Err(err) => return Err(err),
            };

            GameData::Crossword(crossword_data)
        }
        Game::WordLadder => {
            return Err("Failed to load file: Word Search is not yet implemented".to_string());
        }
    };

    let header = Header::new(game, file_checksum);
    let metadata = Metadata::new(title, description, author, creation_date, gamedata_checksum);
    let footer = Footer::new(file_checksum);

    Ok(TggFile {
        header,
        metadata,
        gamedata,
        footer,
    })
}
