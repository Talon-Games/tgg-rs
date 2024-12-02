use std::path::Path;
use tgg::TggFile;

#[test]
fn test_load_crossword() {
    let path = Path::new("./tests/crosswords/crossword.tgg");

    let tgg_file = TggFile::load(path);

    assert!(
        tgg_file.is_ok(),
        "Failed to comment on program: {:?}",
        tgg_file.unwrap_err()
    );
}
