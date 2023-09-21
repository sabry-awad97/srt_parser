use crate::{time_range::TimeRange, traits::Parseable, validation::is_valid_input, ParseError};

#[derive(Debug)]
pub struct Subtitle {
    pub index: u32,
    pub time_range: TimeRange,
    pub text: String,
}

impl Parseable for Subtitle {
    fn parse(input: &str) -> Result<Self, ParseError> {
        if !is_valid_input(input) {
            return Err(ParseError::InvalidFormat("Invalid input".to_string()));
        }

        let parts: Vec<_> = input.lines().collect();
        if parts.len() < 3 {
            return Err(ParseError::MissingFields);
        }

        let index = parts[0].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Failed to parse subtitle index: {}", parts[0]))
        })?;

        if index == 0 {
            return Err(ParseError::InvalidFormat(format!(
                "Subtitle index must be a positive integer, found: {}",
                parts[0]
            )));
        }

        let time_range = Parseable::parse(parts[1])?;
        let text = parts[2..].join(" ");

        Ok(Self {
            index,
            time_range,
            text,
        })
    }
}
