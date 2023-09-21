# SRT Parser

This repository contains the Rust implementation for parsing SRT (SubRip Subtitle) files. The SRT format is a widely used file format for storing subtitles alongside video content.

## Table of Contents

- [Introduction](#introduction)
- [SRT Structure](#srt-structure)
- [Parsing Algorithm](#parsing-algorithm)
- [Usage](#usage)
- [Example](#example)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This Rust crate provides functionalities to parse SRT files, allowing easy integration with your projects that involve subtitles. The crate is designed for efficiency and ease of use.

## SRT Structure

An SRT file consists of numbered subtitle entries, each containing three parts:

1. **Index**: A unique identifier for the subtitle entry.
2. **Timecodes**: The start and end time of the subtitle in the format `hh:mm:ss,ms` --> `hh:mm:ss,ms`.
3. **Text**: Subtitle text in one or more lines.

Here is an example of the SRT file structure:

```txt
1
00:00:01,000 --> 00:00:04,000
Example subtitle line 1.

2
00:00:05,000 --> 00:00:07,500
Example subtitle line 2.
```

## Parsing Algorithm

The parsing algorithm for SRT files in Rust involves the following steps:

1. Read the SRT file line by line.
2. Split the lines into subtitle entries using an empty line as a separator.
3. For each subtitle entry:
   - Extract and store the index.
   - Parse the timecodes into start and end timestamps.
   - Capture the subtitle text.
4. Store each subtitle entry in a suitable data structure.

## Usage

To use this crate in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
srt_parser = { git = "https://github.com/sabry-awad97/srt_parser" }
```

In your Rust code, import the crate and use the provided functions to parse SRT files.

```rust
use srt_parser::parse_srt_file;

fn main() {
    let subtitles = parse_srt_file("path/to/your/subtitles.srt").unwrap();
    // Use the 'subtitles' data structure as needed.
}
```

## Example

For a complete example, refer to the [`examples`](./examples) directory in this repository.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
