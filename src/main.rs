use std::path::Path;
use tgg::crossword::{CrosswordBox, CrosswordBoxValue, CrosswordClue};
use tgg::TggFile;

pub fn main() {
    // let mut horizontal_clues: Vec<CrosswordClue> = Vec::new();
    // let mut vertical_clues: Vec<CrosswordClue> = Vec::new();
    // let mut crossword_data: Vec<Vec<CrosswordBox>> = Vec::new();
    //
    // horizontal_clues.push(CrosswordClue::new(1, "XY"));
    // horizontal_clues.push(CrosswordClue::new(4, "Front of a building"));
    // horizontal_clues.push(CrosswordClue::new(5, "Very small"));
    // horizontal_clues.push(CrosswordClue::new(6, "Shrek"));
    // horizontal_clues.push(CrosswordClue::new(
    //     7,
    //     "Distinguished Service Order, for short",
    // ));
    //
    // vertical_clues.push(CrosswordClue::new(1, "Wide-angle lens setting"));
    // vertical_clues.push(CrosswordClue::new(2, "Unit for land measurement"));
    // vertical_clues.push(CrosswordClue::new(3, "The main character from The Matrix"));
    // vertical_clues.push(CrosswordClue::new(4, "Fruits of a ficus tree"));
    // vertical_clues.push(CrosswordClue::new(5, "To \"Alter\", informally"));
    //
    // let mut row_one: Vec<CrosswordBox> = Vec::new();
    // row_one.push(CrosswordBox::new(0, CrosswordBoxValue::Solid).unwrap());
    // row_one.push(CrosswordBox::new(0, CrosswordBoxValue::Solid).unwrap());
    // row_one.push(CrosswordBox::new(1, CrosswordBoxValue::Letter('M')).unwrap());
    // row_one.push(CrosswordBox::new(2, CrosswordBoxValue::Letter('A')).unwrap());
    // row_one.push(CrosswordBox::new(3, CrosswordBoxValue::Letter('N')).unwrap());
    //
    // let mut row_two: Vec<CrosswordBox> = Vec::new();
    // row_two.push(CrosswordBox::new(0, CrosswordBoxValue::Solid).unwrap());
    // row_two.push(CrosswordBox::new(4, CrosswordBoxValue::Letter('F')).unwrap());
    // row_two.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('A')).unwrap());
    // row_two.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('C')).unwrap());
    // row_two.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('E')).unwrap());
    //
    // let mut row_three: Vec<CrosswordBox> = Vec::new();
    // row_three.push(CrosswordBox::new(5, CrosswordBoxValue::Letter('M')).unwrap());
    // row_three.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('I')).unwrap());
    // row_three.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('C')).unwrap());
    // row_three.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('R')).unwrap());
    // row_three.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('O')).unwrap());
    //
    // let mut row_four: Vec<CrosswordBox> = Vec::new();
    // row_four.push(CrosswordBox::new(6, CrosswordBoxValue::Letter('O')).unwrap());
    // row_four.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('G')).unwrap());
    // row_four.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('R')).unwrap());
    // row_four.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('E')).unwrap());
    // row_four.push(CrosswordBox::new(0, CrosswordBoxValue::Solid).unwrap());
    //
    // let mut row_five: Vec<CrosswordBox> = Vec::new();
    // row_five.push(CrosswordBox::new(7, CrosswordBoxValue::Letter('D')).unwrap());
    // row_five.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('S')).unwrap());
    // row_five.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('O')).unwrap());
    // row_five.push(CrosswordBox::new(0, CrosswordBoxValue::Solid).unwrap());
    // row_five.push(CrosswordBox::new(0, CrosswordBoxValue::Solid).unwrap());
    //
    // crossword_data.push(row_one);
    // crossword_data.push(row_two);
    // crossword_data.push(row_three);
    // crossword_data.push(row_four);
    // crossword_data.push(row_five);
    //
    // let tgg_file = match TggFile::create_for_crossword(
    //     "Test Crossword",
    //     "Just doing some testing",
    //     "Maksim Straus",
    //     5,
    //     5,
    //     horizontal_clues,
    //     vertical_clues,
    //     crossword_data,
    // ) {
    //     Ok(tgg_file) => tgg_file,
    //     Err(err) => {
    //         println!("{}", err);
    //         std::process::exit(0);
    //     }
    // };
    //
    // let path = Path::new("./crossword.tgg");
    //
    // match tgg_file.save(path) {
    //     Ok(_) => println!("Saved!"),
    //     Err(err) => println!("{}", err),
    // };

    let path = Path::new("./tests/crosswords/crossword.tgg");

    let tgg_file = match TggFile::load_from_file(path) {
        Ok(tgg_file) => tgg_file,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    println!("{}", tgg_file.get_title());
}
