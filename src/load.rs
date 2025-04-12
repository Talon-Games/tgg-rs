use crate::{
    crossword::CrosswordData,
    utils::{calculate_checksum, extract_cstring_with_offset},
    Error, Footer, Game, GameData, Header, Metadata, TggFile,
};

pub fn load(bytes: Vec<u8>) -> Result<TggFile, Error> {
    // Validate and extract header
    if bytes.len() < 17 {
        return Err(Error::InsufficientHeaderBytes {
            min: 17,
            found: bytes.len() as u32,
        });
    }

    let header_bytes = &bytes[0..17];

    let id: String = header_bytes[0..14]
        .iter()
        .map(|&byte| byte as char)
        .collect();

    if id != "TalonGamesGame" {
        return Err(Error::InvalidID);
    }

    let game = match Game::from_byte(header_bytes[14]) {
        Some(game) => game,
        None => {
            return Err(Error::InvalidGameTypeByte {
                found: header_bytes[14],
            });
        }
    };
    let file_checksum = u16::from_le_bytes([header_bytes[15], header_bytes[16]]);

    let body = &bytes[17..bytes.len() - 2];

    // Extract metadata
    let (title, offset) = extract_cstring_with_offset(body, 0);
    let (description, offset) = extract_cstring_with_offset(body, offset);
    let (author, offset) = extract_cstring_with_offset(body, offset);

    if title.is_empty() {
        return Err(Error::TitleIsEmpty);
    }

    if description.is_empty() {
        return Err(Error::DescriptionIsEmpty);
    }

    if author.is_empty() {
        return Err(Error::AuthorIsEmpty);
    }

    // Validate metadata boundaries
    if offset + 6 > body.len() {
        return Err(Error::InsufficientMetadataBytes {
            expected: (offset + 6) as u32,
            found: body.len() as u32,
        });
    }

    let creation_date_bytes = &body[offset..offset + 4];
    let creation_date = u32::from_be_bytes(creation_date_bytes.try_into().unwrap());

    let checksum_bytes = &body[offset + 4..offset + 6];
    let gamedata_checksum = u16::from_le_bytes(checksum_bytes.try_into().unwrap());

    // Verify checksums
    let calculated_checksum = u16::from_le_bytes(calculate_checksum(body.to_vec()));
    let footer_file_checksum = u16::from_le_bytes([bytes[bytes.len() - 2], bytes[bytes.len() - 1]]);
    if file_checksum != calculated_checksum {
        return Err(Error::HeaderChecksumMismatch {
            expected: file_checksum,
            found: calculated_checksum,
        });
    }
    if file_checksum != footer_file_checksum {
        return Err(Error::HeaderChecksumMismatch {
            expected: file_checksum,
            found: footer_file_checksum,
        });
    }

    let game_data = &body[offset + 6..body.len()];
    if game_data.is_empty() {
        return Err(Error::GameDataIsEmpty);
    }

    let gamedata: GameData = match game {
        Game::Crossword => {
            let crossword_data = match CrosswordData::load(game_data) {
                Ok(crossword_data) => crossword_data,
                Err(err) => return Err(err),
            };

            GameData::Crossword(crossword_data)
        }
        Game::WordSearch => {
            unimplemented!()
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
