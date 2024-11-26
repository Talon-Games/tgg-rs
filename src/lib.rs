pub mod crossword;
mod load;
mod utils;

use self::crossword::{CrosswordBox, CrosswordClue, CrosswordData};
use self::load::load;
use self::utils::calculate_checksum;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const VERSION: &str = "0.1.0";
const ID: &str = "TalonGamesGame";

pub struct TggFile {
    header: Header,
    metadata: Metadata,
    gamedata: GameData,
    footer: Footer,
}

impl TggFile {
    pub fn load(path: &Path) -> Result<(), String> {
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

        Ok(())
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

        let header = Header::new(Game::Crossword, u16::from_le_bytes(file_checksum));

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

    pub fn game_name(&self) -> String {
        return self.header.game.to_string();
    }
}

pub struct Header {
    version: String,
    id: String,
    pub game: Game,
    file_checksum: u16,
}

impl Header {
    pub fn new(game: Game, file_checksum: u16) -> Header {
        Header {
            version: VERSION.to_string(),
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
    title: String,
    description: String,
    author: String,
    creation_date: u32,
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

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// Function to convert a 4-byte big-endian timestamp into a human-readable date
fn format_timestamp(timestamp: u32) -> String {
    let duration_since_epoch = Duration::from_secs(timestamp as u64);

    // Total days since the UNIX epoch (1970-01-01)
    let days_since_epoch = duration_since_epoch.as_secs() / 86400;

    // Approximate the year
    let mut year = 1970;
    let mut remaining_days = days_since_epoch;

    while remaining_days >= if is_leap_year(year) { 366 } else { 365 } {
        remaining_days -= if is_leap_year(year) { 366 } else { 365 };
        year += 1;
    }

    let month_lengths = [
        31,                                       // January
        if is_leap_year(year) { 29 } else { 28 }, // February
        31,                                       // March
        30,                                       // April
        31,                                       // May
        30,                                       // June
        31,                                       // July
        31,                                       // August
        30,                                       // September
        31,                                       // October
        30,                                       // November
        31,                                       // December
    ];

    let mut month = 1;
    for &days_in_month in &month_lengths {
        if remaining_days < days_in_month {
            break;
        }
        remaining_days -= days_in_month;
        month += 1;
    }

    let day = remaining_days + 1; // Convert 0-based to 1-based day

    // Convert month number to name
    let month_name = match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Month",
    };

    format!("{}, {:02}, {}", month_name, day, year).to_string()
}
