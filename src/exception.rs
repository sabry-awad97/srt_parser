use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("Invalid timecode: {0}")]
    InvalidTimecode(String),
    #[error("Missing fields")]
    MissingFields,
}
