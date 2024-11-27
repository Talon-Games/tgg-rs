use crate::utils::extract_cstring_with_offset;
use std::usize;

pub struct CrosswordData {
    width: u8,
    height: u8,
    total_clues: u8,
    horizontal_clues: Vec<CrosswordClue>,
    vertical_clues: Vec<CrosswordClue>,
    crossword_data: Vec<Vec<CrosswordBox>>,
}
//TODO: fully validate crossword
impl CrosswordData {
    pub fn load(bytes: &[u8]) -> Result<(), &'static str> {
        let mut offset = 0;
        let width = bytes[offset];
        offset += 1;
        let height = bytes[offset];
        offset += 1;
        let total_clues = bytes[offset];
        offset += 1;

        // Parse horizontal clues
        let mut horizontal_clues = Vec::new();
        while bytes[offset] != 0x00 {
            let (clue, new_offset) = parse_crossword_clue(bytes, offset);
            horizontal_clues.push(clue);
            offset = new_offset;
        }
        offset += 1; // Skip the 0x00 separator

        // Parse vertical clues
        let mut vertical_clues = Vec::new();
        while bytes[offset] != 0x00 {
            let (clue, new_offset) = parse_crossword_clue(bytes, offset);
            vertical_clues.push(clue);
            offset = new_offset;
        }
        offset += 1; // Skip the 0x00 separator

        if horizontal_clues.len() + vertical_clues.len() != total_clues as usize {
            return Err("Amount of clues did not match total of horizontal and vertical clues");
        };

        // Multiply the product of width and height by 2 to account for the number byte with every char
        if offset as usize + (width * height) as usize * 2 != bytes.len() {
            return Err(
                "Amount of data does not match expected amount based on crossword width and height",
            );
        }

        Ok(())
    }

    pub fn new(
        width: u8,
        height: u8,
        horizontal_clues: Vec<CrosswordClue>,
        vertical_clues: Vec<CrosswordClue>,
        crossword_data: Vec<Vec<CrosswordBox>>,
    ) -> Result<CrosswordData, &'static str> {
        // Validate crossword size
        if crossword_data.len() != height as usize {
            return Err("Height of crossword did not match height of crossword data");
        }

        for row in &crossword_data {
            if row.len() != width as usize {
                return Err("Width of crossword did not match width of crossword data");
            }
        }

        // Validate crossword numbers
        let mut numbers_in_crossword: Vec<u8> = Vec::new();
        for row in &crossword_data {
            for item in row {
                if numbers_in_crossword.contains(&item.number) && item.number != 0 {
                    return Err("Crossword contains duplicate numbers");
                }

                numbers_in_crossword.push(item.number);
            }
        }

        // Validate clues
        let mut clue_numbers: Vec<u8> = Vec::new();
        for clue in &vertical_clues {
            if !numbers_in_crossword.contains(&clue.number) {
                return Err("A vertical clue contains a number not found in the crossword");
            }

            if clue_numbers.contains(&clue.number) {
                return Err("A vertical clue contains a duplicate number");
            }

            clue_numbers.push(clue.number);
        }

        clue_numbers.clear();

        // Horizontal clues
        for clue in &horizontal_clues {
            if !numbers_in_crossword.contains(&clue.number) {
                return Err("A horizontal clue contains a number not found in the crossword");
            }

            if clue_numbers.contains(&clue.number) {
                return Err("A horizontal clue contains a duplicate number");
            }

            clue_numbers.push(clue.number);
        }

        Ok(CrosswordData {
            width,
            height,
            total_clues: horizontal_clues.len() as u8 + vertical_clues.len() as u8,
            vertical_clues,
            horizontal_clues,
            crossword_data,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(self.width);
        bytes.push(self.height);
        bytes.push(self.total_clues);

        for clue in &self.horizontal_clues {
            bytes.extend(clue.to_bytes());
        }

        bytes.push(0);

        for clue in &self.vertical_clues {
            bytes.extend(clue.to_bytes());
        }

        bytes.push(0);

        for row in &self.crossword_data {
            for item in row {
                bytes.extend(item.to_bytes());
            }
        }

        return bytes;
    }
}

fn parse_crossword_clue(bytes: &[u8], start: usize) -> (CrosswordClue, usize) {
    let number = bytes[start];
    let (value, end_offset) = extract_cstring_with_offset(bytes, start + 1); // Start after the clue number
    let clue = CrosswordClue::new(number, &value);
    (clue, end_offset)
}

pub struct CrosswordClue {
    pub number: u8,
    pub value: String,
}

impl CrosswordClue {
    pub fn new(number: u8, value: &str) -> CrosswordClue {
        CrosswordClue {
            number,
            value: value.to_string(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(self.number);
        bytes.extend(self.value.as_bytes());
        bytes.push(0);
        bytes
    }
}

pub struct CrosswordBox {
    pub number: u8,
    pub letter: CrosswordBoxValue,
}

impl CrosswordBox {
    pub fn new(number: u8, letter: CrosswordBoxValue) -> Result<CrosswordBox, &'static str> {
        match letter {
            CrosswordBoxValue::Letter(letter) => {
                if !letter.is_ascii() {
                    return Err("Failed to create crossword box, letter must be ASCII");
                }

                if !letter.is_alphabetic() {
                    return Err("Failed to create crossword box, letter must be alphabetic");
                }

                if letter.is_lowercase() {
                    return Err("Failed to create crossword box, letter must be uppercase");
                }
            }
            _ => {}
        }

        Ok(CrosswordBox { number, letter })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.number);
        bytes.push(self.letter.to_byte());

        return bytes;
    }
}

pub enum CrosswordBoxValue {
    Empty,
    Solid,
    Letter(char),
}

impl CrosswordBoxValue {
    pub fn to_byte(&self) -> u8 {
        match self {
            CrosswordBoxValue::Empty => 0x20,
            CrosswordBoxValue::Solid => 0x23,
            CrosswordBoxValue::Letter(letter) => *letter as u8,
        }
    }
}
