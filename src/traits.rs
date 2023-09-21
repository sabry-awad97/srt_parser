use crate::ParseError;

pub trait Parseable {
    fn parse(input: &str) -> Result<Self, ParseError>
    where
        Self: Sized;
}
