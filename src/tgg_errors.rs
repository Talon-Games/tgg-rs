use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TggError {
    InvalidExtension,
    FileNotFound,
    FileAlreadyExists,
    InvalidPath,
    IoError(io::Error), // Wraps I/O errors
    InvalidHeader(String),
    MetadataError(String),
    ChecksumMismatch,
    UnsupportedGameType(u8),
    CrosswordError(String),
    Generic(String), // Catch-all for other errors
}

impl fmt::Display for TggError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TggError::InvalidExtension => write!(f, "File must have a .tgg extension"),
            TggError::FileNotFound => write!(f, "File not found"),
            TggError::FileAlreadyExists => write!(f, "File already exists"),
            TggError::InvalidPath => write!(f, "Invalid file path"),
            TggError::IoError(err) => write!(f, "I/O error: {}", err),
            TggError::InvalidHeader(reason) => write!(f, "Invalid header: {}", reason),
            TggError::MetadataError(reason) => write!(f, "Metadata error: {}", reason),
            TggError::ChecksumMismatch => write!(f, "Checksum mismatch"),
            TggError::UnsupportedGameType(byte) => {
                write!(f, "Unsupported game type: {:#04X}", byte)
            }
            TggError::CrosswordError(reason) => write!(f, "Crossword error: {}", reason),
            TggError::Generic(reason) => write!(f, "Error: {}", reason),
        }
    }
}

impl std::error::Error for TggError {}

impl From<io::Error> for TggError {
    fn from(err: io::Error) -> TggError {
        TggError::IoError(err)
    }
}
