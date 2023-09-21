use crate::{timecode::Timecode, traits::Parseable, validation::is_valid_input, ParseError};

#[derive(Debug)]
pub struct TimeRange {
    pub start: Timecode,
    pub end: Timecode,
}

impl Parseable for TimeRange {
    fn parse(input: &str) -> Result<Self, ParseError> {
        if !is_valid_input(input) {
            return Err(ParseError::InvalidFormat("Invalid input".to_string()));
        }

        let parts: Vec<_> = input.trim().split(" --> ").collect();
        if parts.len() != 2 {
            return Err(ParseError::InvalidFormat(format!(
                "Expected 2 parts, found {}",
                parts.len()
            )));
        }

        let start = Parseable::parse(parts[0])?;
        let end = Parseable::parse(parts[1])?;

        Ok(Self { start, end })
    }
}
