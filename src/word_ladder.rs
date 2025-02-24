use crate::utils::extract_cstring_with_offset;

#[derive(Debug)]
pub struct WordLadder {
    pub starting_word: String,
    pub starting_word_def: String,
    pub ending_word: String,
    pub ending_word_def: String,
    pub steps: u8,
    pub step_hints: Vec<String>,
}

impl WordLadder {
    pub fn load(bytes: &[u8]) -> Result<WordLadder, String> {
        let (starting_word, offset) = extract_cstring_with_offset(bytes, 0);
        let (starting_word_def, offset) = extract_cstring_with_offset(bytes, offset);
        let (ending_word, offset) = extract_cstring_with_offset(bytes, offset);
        let (ending_word_def, offset) = extract_cstring_with_offset(bytes, offset);

        // TODO: get the steps and add 1 to offset at next parse
        //

        let steps: u8 = 0;

        let mut step_hints: Vec<String> = Vec::new();

        Ok(WordLadder {
            starting_word,
            starting_word_def,
            ending_word,
            ending_word_def,
            steps,
            step_hints,
        })
    }
}
