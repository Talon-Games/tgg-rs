#[derive(Debug)]
pub enum Error {
    // -- Load
    InsufficientHeaderBytes { min: u32, found: u32 },
    InvalidID,
    InvalidGameTypeByte { found: u8 },
    TitleIsEmpty,
    DescriptionIsEmpty,
    AuthorIsEmpty,
    InsufficientMetadataBytes { expected: u32, found: u32 },
    HeaderChecksumMismatch { expected: u16, found: u16 },
    FooterChecksumMismatch { expected: u16, found: u16 },
    GameDataIsEmpty,
    // -- Crossword
    UnexpectedEndOfFile,
    WidthOrHeightIsZero,
    TotalCluesIsZero,
    ClueCountMismatch { expected: u8, found: u32 },
    NotEnoughCrosswordBytes { expected: u32, found: u32 },
    HeightCrosswordDataMismatch { height: u8, crossword_height: u32 },
    WidthCrosswordDataMismatch { width: u8, crossword_width: u32 },
    DuplicateNumber { number: u8 },
    VerticalClueContainsInvalidNumber { number: u8 },
    VerticalClueContainsDuplicate { number: u8 },
    HorizontalClueContainsInvalidNumber { number: u8 },
    HorizontalClueContainsDuplicate { number: u8 },
    NonAsciiCharacter,
    NonAlphabeticCharacter,
    NonUppercaseCharacter,
    InvalidCrosswordBoxByte { found: u8 },
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
