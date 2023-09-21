use crate::{subtitle::Subtitle, ParseError};

pub trait Parseable {
    fn parse(input: &str) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait SubtitleParser {
    fn parse(srt_content: &str) -> Result<Vec<Subtitle>, Vec<ParseError>>;
}
