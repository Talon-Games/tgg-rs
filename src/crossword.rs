use crate::utils::extract_cstring_with_offset;
use std::usize;

#[derive(Debug)]
pub struct CrosswordData {
    pub width: u8,
    pub height: u8,
    pub total_clues: u8,
    pub horizontal_clues: Vec<CrosswordClue>,
    pub vertical_clues: Vec<CrosswordClue>,
    pub crossword_data: Vec<Vec<CrosswordBox>>,
}

impl CrosswordData {
    pub fn load(bytes: &[u8]) -> Result<CrosswordData, String> {
        let mut offset = 0;
        if bytes.len() < 3 {
            return Err("Unexpected end of file while parsing".to_string());
        }
        let width = bytes[offset];
        offset += 1;
        let height = bytes[offset];
        offset += 1;
        let total_clues = bytes[offset];
        offset += 1;

        if width == 0 || height == 0 {
            return Err(
                "Invalid crossword dimensions: width and height must be non-zero".to_string(),
            );
        }

        if total_clues == 0 {
            return Err("Invalid crossword: total clues must be greater than zero".to_string());
        }

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
            return Err(format!(
                "Failed to load Crossword: expected {} clues, found {} clues",
                total_clues,
                horizontal_clues.len() + vertical_clues.len()
            ));
        };

        let expected_bytes = offset as usize + (width * height) as usize * 2;

        // Multiply the product of width and height by 2 to account for the number byte with every char
        if expected_bytes != bytes.len() {
            return Err(format!("Failed to load Crossword: based on width and height, {} bytes are expected, but {} bytes are found", expected_bytes, bytes.len()));
        }

        let mut crossword_data: Vec<Vec<CrosswordBox>> = Vec::new();

        for _ in 0..height {
            let mut row: Vec<CrosswordBox> = Vec::new();
            for _ in 0..width {
                let number = bytes[offset];
                let value = match CrosswordBoxValue::from_byte(bytes[offset + 1]) {
                    Ok(value) => value,
                    Err(err) => return Err(err),
                };
                let crossword_box = match CrosswordBox::new(number, value) {
                    Ok(crossword_box) => crossword_box,
                    Err(err) => return Err(err.to_string()),
                };
                row.push(crossword_box);
                offset += 2;
            }
            crossword_data.push(row);
        }

        Ok(CrosswordData {
            width,
            height,
            total_clues,
            horizontal_clues,
            vertical_clues,
            crossword_data,
        })
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CrosswordBox {
    pub number: u8,
    pub value: CrosswordBoxValue,
}

impl CrosswordBox {
    pub fn new(number: u8, value: CrosswordBoxValue) -> Result<CrosswordBox, &'static str> {
        match value {
            CrosswordBoxValue::Letter(value) => {
                if !value.is_ascii() {
                    return Err("Failed to create crossword box, letter must be ASCII");
                }

                if !value.is_alphabetic() {
                    return Err("Failed to create crossword box, letter must be alphabetic");
                }

                if value.is_lowercase() {
                    return Err("Failed to create crossword box, letter must be uppercase");
                }
            }
            _ => {}
        }

        Ok(CrosswordBox { number, value })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.number);
        bytes.push(self.value.to_byte());

        return bytes;
    }
}

#[derive(Debug)]
pub enum CrosswordBoxValue {
    Empty,
    Solid,
    Letter(char),
}

impl CrosswordBoxValue {
    pub fn to_byte(&self) -> u8 {
        match self {
            CrosswordBoxValue::Empty => 0x20, // ASCII for space
            CrosswordBoxValue::Solid => 0x23, // ASCII for #
            CrosswordBoxValue::Letter(letter) => *letter as u8,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CrosswordBoxValue::Empty => " ".to_string(),
            CrosswordBoxValue::Solid => "#".to_string(),
            CrosswordBoxValue::Letter(letter) => letter.to_string(),
        }
    }

    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0x20 => Ok(CrosswordBoxValue::Empty), // ASCII for space
            0x23 => Ok(CrosswordBoxValue::Solid), // ASCII for #
            b if b.is_ascii_alphabetic() => Ok(CrosswordBoxValue::Letter(b as char)),
            _ => Err(format!("Invalid byte for CrosswordBoxValue: {:02X}", byte)),
        }
    }
}

fn parse_crossword_clue(bytes: &[u8], start: usize) -> (CrosswordClue, usize) {
    let number = bytes[start];
    let (value, end_offset) = extract_cstring_with_offset(bytes, start + 1); // Start after the clue number
    let clue = CrosswordClue::new(number, &value);
    (clue, end_offset)
}
