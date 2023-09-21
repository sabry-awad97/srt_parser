use std::{fs, io, path::Path};

/// Reads the content of a file and returns it as a string.
///
/// # Arguments
///
/// * `path` - A path to the file.
///
/// # Returns
///
/// * `Result<String, io::Error>` - A Result containing the file content as a string, or an IO error.
///
/// # Example
///
/// ```
/// # use crate::utils::from_file;
/// let content = from_file("path/to/your/subtitles.srt");
/// assert!(content.is_ok());
/// ```
pub fn from_file<P>(path: P) -> Result<String, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
}
