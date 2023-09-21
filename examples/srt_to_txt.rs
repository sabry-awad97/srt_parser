use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use srt_parser::parse_srt_file;

/// Retrieves a list of SRT files from the current directory.
///
/// # Returns
///
/// * `Result<Vec<PathBuf>, io::Error>` - A Result containing a vector of SRT file paths or an IO error.
///
/// # Example
///
/// ```
/// let result = get_srt_files();
/// assert!(result.is_ok());
/// ```
fn get_srt_files() -> Result<Vec<PathBuf>, io::Error> {
    let srt_extension = "srt";

    let current_dir = std::env::current_dir()?;
    let file_list: Vec<_> = current_dir
        .read_dir()?
        .filter_map(|entry| entry.ok())
        .collect();

    let srt_files: Vec<PathBuf> = file_list
        .iter()
        .filter(|entry| entry.path().extension() == Some(OsStr::new(srt_extension)))
        .map(|entry| entry.path())
        .collect();

    Ok(srt_files)
}

/// Converts an SRT file to plain text format.
///
/// # Arguments
///
/// * `srt_file_path` - The path to the SRT file.
/// * `output_path` - The path where the plain text file will be saved.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` if the conversion is successful, otherwise an error.
///
/// # Example
///
/// ```
/// # use srt_parser::srt_to_txt;
/// let result = srt_to_txt("path/to/your/input.srt", "path/to/your/output.txt");
/// assert!(result.is_ok());
/// ```
fn srt_to_txt(srt_file_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let subtitles = match parse_srt_file(srt_file_path) {
        Ok(subtitles) => subtitles,
        Err(errors) => {
            for error in errors {
                eprintln!("Error: {}", error);
            }
            return Ok(());
        }
    };

    let mut output_file = File::create(output_path)?;
    for subtitle in subtitles {
        writeln!(output_file, "{}", subtitle.text)?;
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let srt_files = get_srt_files()?;
    for srt_file in srt_files {
        let txt_output_path = format!("{}.txt", srt_file.file_stem().unwrap().to_str().unwrap());
        srt_to_txt(srt_file.to_str().unwrap(), &txt_output_path)?;
        println!("Conversion successful for {}", srt_file.display());
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
