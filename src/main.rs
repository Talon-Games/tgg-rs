use std::path::Path;
use tgg::crossword::{CrosswordBox, CrosswordBoxValue, CrosswordClue, Direction};
use tgg::TggFile;

/**
* C A T
* # # A
* # # B
*/
pub fn main() {
    let mut horizontal_clues: Vec<CrosswordClue> = Vec::new();
    let mut vertical_clues: Vec<CrosswordClue> = Vec::new();
    let mut crossword_data: Vec<Vec<CrosswordBox>> = Vec::new();

    horizontal_clues.push(CrosswordClue::new(1, Direction::Across, "Good pet"));
    vertical_clues.push(CrosswordClue::new(2, Direction::Down, "Starts a paragraph"));

    let mut row_one: Vec<CrosswordBox> = Vec::new();
    row_one.push(CrosswordBox::new(1, CrosswordBoxValue::Letter('C')).expect("bad 1 1"));
    row_one.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('A')).expect("bad 1 2"));
    row_one.push(CrosswordBox::new(2, CrosswordBoxValue::Letter('T')).expect("bad 1 3"));

    let mut row_two: Vec<CrosswordBox> = Vec::new();
    row_two.push(CrosswordBox::new(0, CrosswordBoxValue::Solid).expect("bad 2 1"));
    row_two.push(CrosswordBox::new(0, CrosswordBoxValue::Solid).expect("bad 2 2"));
    row_two.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('A')).expect("bad 2 3"));

    let mut row_three: Vec<CrosswordBox> = Vec::new();
    row_three.push(CrosswordBox::new(0, CrosswordBoxValue::Solid).expect("bad 3 1"));
    row_three.push(CrosswordBox::new(0, CrosswordBoxValue::Solid).expect("bad 3 2"));
    row_three.push(CrosswordBox::new(0, CrosswordBoxValue::Letter('B')).expect("bad 3 3"));

    crossword_data.push(row_one);
    crossword_data.push(row_two);
    crossword_data.push(row_three);

    let tgg_file = match TggFile::create_for_crossword(
        "Test Crossword",
        "Just doing some testing",
        "Maksim Straus",
        3,
        3,
        horizontal_clues,
        vertical_clues,
        crossword_data,
    ) {
        Ok(tgg_file) => tgg_file,
        Err(err) => {
            println!("{}", err);
            std::process::exit(0);
        }
    };

    let path = Path::new("./crossword.tgg");

    match tgg_file.save(path) {
        Ok(_) => println!("Saved!"),
        Err(err) => println!("{}", err),
    };
}
