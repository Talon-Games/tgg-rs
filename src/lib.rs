pub mod crossword;
mod date;
mod load;
mod utils;

use crate::crossword::{CrosswordBox, CrosswordClue, CrosswordData};
use crate::date::format_timestamp;
use crate::load::load;
use crate::utils::calculate_checksum;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const VERSION: &str = "0.1.0";
const ID: &str = "TalonGamesGame";

pub struct TggFile {
    header: Header,
    metadata: Metadata,
    gamedata: GameData,
    footer: Footer,
}

impl TggFile {
    pub fn load(path: &Path) -> Result<TggFile, String> {
        if let Some(extension) = path.extension() {
            if extension != "tgg" {
                return Err(format!(
                    "Failed to load file: the file must have a .tgg extension"
                ));
            }
        } else {
            return Err("Failed to load file: the file must have a .tgg extension".to_string());
        }

        if !path.exists() {
            return Err(format!(
                "Failed to load file: file does not exist at {}",
                path.display()
            ));
        }

        let bytes: Vec<u8> = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => return Err(format!("Failed to load file: {}", err)),
        };

        let file = match load(bytes) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

        Ok(file)
    }

    pub fn create_for_crossword(
        title: &str,
        description: &str,
        author: &str,
        width: u8,
        height: u8,
        horizontal_clues: Vec<CrosswordClue>,
        vertical_clues: Vec<CrosswordClue>,
        crossword_data: Vec<Vec<CrosswordBox>>,
    ) -> Result<TggFile, &'static str> {
        let crossword = CrosswordData::new(
            width,
            height,
            horizontal_clues,
            vertical_clues,
            crossword_data,
        )?;

        let gamedata_checksum = calculate_checksum(crossword.to_bytes());

        let metadata = Metadata::create(
            title,
            description,
            author,
            u16::from_le_bytes(gamedata_checksum),
        );

        let mut bytes = crossword.to_bytes();
        bytes.extend(metadata.to_bytes());

        let file_checksum = calculate_checksum(bytes);

        let footer = Footer::new(u16::from_le_bytes(file_checksum));

        let header = Header::new(
            VERSION.to_string(),
            Game::Crossword,
            u16::from_le_bytes(file_checksum),
        );

        Ok(TggFile {
            header,
            metadata,
            gamedata: GameData::Crossword(crossword),
            footer,
        })
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(extension) = path.extension() {
            if extension != "tgg" {
                return Err(format!(
                    "Failed to save file: the file must have a .tgg extension"
                ));
            }
        } else {
            return Err("Failed to save file: the file must have a .tgg extension".to_string());
        }

        if path.exists() {
            return Err(format!(
                "Failed to save file: file already exists at {}",
                path.display()
            ));
        }

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                // Create the parent directory if it doesn't exist
                if let Err(e) = fs::create_dir_all(parent) {
                    return Err(format!("Failed to create parent directory: {}", e));
                }
            }
        } else {
            return Err("Failed to save file: invalid file path".to_string());
        }

        let contents = self.to_bytes();

        fs::write(path, contents).map_err(|e| format!("Failed to save file: {}", e))?;

        Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.header.to_bytes());
        bytes.extend(self.metadata.to_bytes());
        bytes.extend(self.gamedata.to_bytes());
        bytes.extend(self.footer.to_bytes());

        bytes
    }

    pub fn get_game_name(&self) -> String {
        return self.header.game.to_string();
    }

    pub fn get_title(&self) -> String {
        return self.metadata.title.to_string();
    }

    pub fn get_description(&self) -> String {
        return self.metadata.description.to_string();
    }

    pub fn get_author(&self) -> String {
        return self.metadata.author.to_string();
    }

    pub fn get_raw_creation_date(&self) -> u32 {
        return self.metadata.creation_date;
    }

    pub fn get_formatted_creation_date(&self) -> String {
        return self.metadata.get_date();
    }

    pub fn get_game_data<'a>(&'a self) -> &'a GameData {
        return &self.gamedata;
    }
}

pub struct Header {
    version: String,
    id: String,
    pub game: Game,
    file_checksum: u16,
}

impl Header {
    pub fn new(version: String, game: Game, file_checksum: u16) -> Header {
        Header {
            version,
            id: ID.to_string(),
            game,
            file_checksum,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.version.as_bytes());
        bytes.extend(self.id.as_bytes());
        bytes.push(self.game.to_byte());
        bytes.extend(self.file_checksum.to_le_bytes());

        bytes
    }
}

pub enum Game {
    Crossword,
    WordSearch,
}

impl Game {
    pub fn to_byte(&self) -> u8 {
        match self {
            Game::Crossword => return 0x01,
            Game::WordSearch => return 0x02,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Game::Crossword => "Crossword",
            Game::WordSearch => "Word Search",
        }
        .to_string()
    }

    pub fn from_byte(byte: u8) -> Option<Game> {
        match byte {
            0x01 => return Some(Game::Crossword),
            0x02 => return Some(Game::WordSearch),
            _ => return None,
        }
    }
}

pub struct Metadata {
    pub title: String,
    pub description: String,
    pub author: String,
    pub creation_date: u32,
    gamedata_checksum: u16,
}

impl Metadata {
    pub fn new(
        title: String,
        description: String,
        author: String,
        creation_date: u32,
        gamedata_checksum: u16,
    ) -> Metadata {
        Metadata {
            title,
            description,
            author,
            creation_date,
            gamedata_checksum,
        }
    }

    pub fn create(
        title: &str,
        description: &str,
        author: &str,
        gamedata_checksum: u16,
    ) -> Metadata {
        let now = SystemTime::now();
        let timestamp = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as u32;

        Metadata {
            title: title.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            creation_date: timestamp,
            gamedata_checksum,
        }
    }

    pub fn get_date(&self) -> String {
        return format_timestamp(self.creation_date);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.title.as_bytes());
        bytes.push(0);
        bytes.extend(self.description.as_bytes());
        bytes.push(0);
        bytes.extend(self.author.as_bytes());
        bytes.push(0);
        bytes.extend(self.creation_date.to_be_bytes());
        bytes.extend(self.gamedata_checksum.to_le_bytes());

        bytes
    }
}

pub enum GameData {
    Crossword(CrosswordData),
}

impl GameData {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            GameData::Crossword(data) => data.to_bytes(),
        }
    }
}

pub struct Footer {
    file_checksum: u16,
}

impl Footer {
    pub fn new(file_checksum: u16) -> Footer {
        Footer { file_checksum }
    }

    pub fn to_bytes(&self) -> [u8; 2] {
        self.file_checksum.to_le_bytes()
    }
}
