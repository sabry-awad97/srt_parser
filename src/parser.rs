use std::path::Path;

use crate::{
    subtitle::Subtitle,
    traits::{Parseable, SubtitleParser},
    utils::from_file,
    ParseError,
};

pub struct SrtParser;

impl SubtitleParser for SrtParser {
    fn parse(srt_content: &str) -> Result<Vec<Subtitle>, Vec<ParseError>> {
        let mut errors = Vec::new();
        let subtitles: Vec<Subtitle> = srt_content
            .replace("\r\n", "\n")
            .split("\n\n")
            .filter_map(|s| match Parseable::parse(s) {
                Ok(subtitle) => Some(subtitle),
                Err(error) => {
                    errors.push(error);
                    None
                }
            })
            .collect();

        if errors.is_empty() {
            Ok(subtitles)
        } else {
            Err(errors)
        }
    }
}

impl SrtParser {
    /// Parses SRT content and returns a vector of Subtitles.
    ///
    /// # Arguments
    ///
    /// * `srt_content` - A string containing SRT formatted content.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Subtitle>)` - A vector of parsed Subtitles.
    /// * `Err(Vec<ParseError>)` - A vector of parsing errors.
    ///
    /// # Example
    ///
    /// ```
    /// # use srt_parser::SrtParser;
    /// let srt_content = "1\n00:00:00,000 --> 00:00:01,000\nSubtitle text";
    /// let result = SrtParser::parse(srt_content);
    /// assert!(result.is_ok());
    /// ```
    pub fn parse(srt_content: &str) -> Result<Vec<Subtitle>, Vec<ParseError>> {
        <Self as SubtitleParser>::parse(srt_content)
    }
}

/// Parses the content of an SRT file and returns a vector of Subtitles.
///
/// # Arguments
///
/// * `path` - A path to the SRT file.
///
/// # Returns
///
/// * `Result<Vec<Subtitle>, Vec<ParseError>>` - A Result containing a vector of Subtitles, or a vector of parsing errors.
///
/// # Example
///
/// ```
/// # use srt_parser::parse_srt_file;
/// let result = parse_srt_file("path/to/your/file.srt");
/// assert!(result.is_ok());
/// ```
pub fn parse_srt_file<P>(path: P) -> Result<Vec<Subtitle>, Vec<ParseError>>
where
    P: AsRef<Path>,
{
    match from_file(path) {
        Ok(content) => SrtParser::parse(&content),
        Err(error) => Err(vec![ParseError::IoError(error)]),
    }
}
