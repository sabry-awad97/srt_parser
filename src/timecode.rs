use crate::{traits::Parseable, validation::is_valid_input, ParseError};

#[derive(Debug)]
pub struct Timecode {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
}

impl Parseable for Timecode {
    fn parse(input: &str) -> Result<Self, ParseError> {
        if !is_valid_input(input) {
            return Err(ParseError::InvalidFormat("Invalid input".to_string()));
        }

        let parts: Vec<_> = input.trim().split(&[':', ',', ' ']).collect();
        if parts.len() != 4 {
            return Err(ParseError::InvalidFormat(format!(
                "Expected 4 parts, found {}",
                parts.len()
            )));
        }

        let hours = parts[0].parse().map_err(|_| {
            ParseError::InvalidTimecode(format!("Failed to parse hours: {}", parts[0]))
        })?;
        let minutes = parts[1].parse().map_err(|_| {
            ParseError::InvalidTimecode(format!("Failed to parse minutes: {}", parts[1]))
        })?;
        let seconds = parts[2].parse().map_err(|_| {
            ParseError::InvalidTimecode(format!("Failed to parse seconds: {}", parts[2]))
        })?;
        let milliseconds = parts[3].parse().map_err(|_| {
            ParseError::InvalidTimecode(format!("Failed to parse milliseconds: {}", parts[3]))
        })?;

        if hours >= 24 || minutes >= 60 || seconds >= 60 || milliseconds >= 1000 {
            return Err(ParseError::InvalidTimecode(format!(
                "Time values out of range: {}:{}:{},{})",
                hours, minutes, seconds, milliseconds
            )));
        }

        Ok(Self {
            hours,
            minutes,
            seconds,
            milliseconds,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_timecode() {
        let input = "12:34:56,789";
        let result = Timecode::parse(input);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
    }

    #[test]
    fn test_invalid_input() {
        let input = "12:34:56,789:";
        let result = Timecode::parse(input);
        assert!(result.is_err(), "Expected Err, got {:?}", result);
    }

    #[test]
    fn test_invalid_parts_count() {
        let input = "12:34:56";
        let result = Timecode::parse(input);
        assert!(result.is_err(), "Expected Err, got {:?}", result);
    }

    #[test]
    fn test_invalid_time_values() {
        let input = "25:61:61,1000";
        let result = Timecode::parse(input);
        assert!(result.is_err(), "Expected Err, got {:?}", result);
    }
}
