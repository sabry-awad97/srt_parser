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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_time_range() {
        let input = "00:00:01,000 --> 00:00:04,000";
        let result = TimeRange::parse(input);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
    }

    #[test]
    fn test_invalid_input() {
        let input = "00:00:01,000 -> 00:00:04,000";
        let result = TimeRange::parse(input);
        assert!(result.is_err(), "Expected Err, got {:?}", result);
    }

    #[test]
    fn test_invalid_parts_count() {
        let input = "00:00:01,000";
        let result = TimeRange::parse(input);
        assert!(result.is_err(), "Expected Err, got {:?}", result);
    }

    #[test]
    fn test_invalid_timecode_format() {
        let input = "00:00:01.000 --> 00:00:04.000";
        let result = TimeRange::parse(input);
        assert!(result.is_err(), "Expected Err, got {:?}", result);
    }

    #[test]
    fn test_invalid_timecode_values() {
        let input = "00:60:60,1000 --> 00:00:04,000";
        let result = TimeRange::parse(input);
        assert!(result.is_err(), "Expected Err, got {:?}", result);
    }
}
