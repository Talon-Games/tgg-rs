pub struct CrosswordData {
    width: u8,
    height: u8,
    total_clues: u8,
    horizontal_clues: Vec<CrosswordClue>,
    vertical_clues: Vec<CrosswordClue>,
    crossword_data: Vec<Vec<CrosswordBox>>,
}

impl CrosswordData {
    pub fn new(
        width: u8,
        height: u8,
        horizontal_clues: Vec<CrosswordClue>,
        vertical_clues: Vec<CrosswordClue>,
        crossword_data: Vec<Vec<CrosswordBox>>,
    ) -> Result<CrosswordData, &'static str> {
        if crossword_data.is_empty() || crossword_data.len() != height as usize {
            return Err("Height of crossword did not match height of crossword data");
        }

        if crossword_data[0].is_empty() || crossword_data[0].len() != width as usize {
            return Err("Width of crossword did not match width of crossword data");
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

pub struct CrosswordClue {
    number: u8,
    direction: Direction,
    value: String,
}

impl CrosswordClue {
    pub fn new(number: u8, direction: Direction, value: &str) -> CrosswordClue {
        CrosswordClue {
            number,
            direction,
            value: value.to_string(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(self.number);
        bytes.push(self.direction.to_byte());
        bytes.extend(self.value.as_bytes());
        bytes.push(0);
        bytes
    }
}

pub struct CrosswordBox {
    number: u8,
    letter: CrosswordBoxValue,
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

pub enum Direction {
    Down,
    Across,
}

impl Direction {
    pub fn to_byte(&self) -> u8 {
        match self {
            Direction::Down => return 0x01,
            Direction::Across => return 0x02,
        }
    }
}
