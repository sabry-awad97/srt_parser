mod exception;
mod parser;
mod subtitle;
mod time_range;
mod timecode;
mod traits;
mod utils;
mod validation;

pub use exception::ParseError;
pub use parser::{parse_srt_file, SrtParser};
