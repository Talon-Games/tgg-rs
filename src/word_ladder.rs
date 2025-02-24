#[derive(Debug)]
pub struct WordLadder {
    pub starting_word: String,
    pub starting_word_def: String,
    pub ending_word: String,
    pub ending_word_def: String,
    pub steps: u8,
    pub step_hints: Vec<String>,
}
