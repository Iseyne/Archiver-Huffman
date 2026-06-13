# Archiver-Huffman [![CI](https://github.com/Iseyne/Archiver-Huffman/actions/workflows/rust.yml/badge.svg)](https://github.com/Iseyne/Archiver-Huffman/actions/workflows/rust.yml) [![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)

> Huffman archiver with canonical codes, written in Rust.  
> Compresses and decompresses binary files using byte-level Huffman coding.  
> Supports compact headers for small alphabets and stores original file size for exact restoration.

## Requirements

- **Rust** 1.85+ (stable toolchain; edition 2024)
- **Cargo** (comes with Rust)
- No external dependencies required

## Table of Contents
- [Installation](#installation)
- [Archive Format](#archive-format)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Example](#example)
- [API](#api)
- [Testing](#testing)
- [Error Types](#error-types)
- [License](#license)
- [Author](#author)

## Installation
```bash
git clone git@github.com:Iseyne/Archiver-Huffman.git
cargo build --release
```

## Archive Format
| Offset | Size         | Description                                         |
|--------|--------------|-----------------------------------------------------|
| 0      | 8 bytes      | Size of input file in bytes (little-endian u64)     |
| 8      | 1 byte       | Number of unique bytes minus one                    |
| 9      | variable     | Table with length of bytes (compact or full)        |
| ...    | ...          | Compressed file info                                |

- If unique bytes < 128: each entry is a pair `(byte, code_length)`.
- Otherwise: 256 bytes, where the `i`-th byte is the code length for symbol `i` (0 = absent).

## Usage
For compress file type
```bash
cargo run zip <path_to_input_file> <path_to_output_file>
```
For decompress file type 
```bash
cargo run unzip <path_to_input_file> <path_to_output_file>
```
You can also don't type ```<path_to_output_file>``` and it could be ```<path_to_input_file>_zipped``` \
or ```<path_to_input_file>_unzipped``` in this directory

## Project Structure
```
src/
├── main.rs                         # CLI entry point
├── lib.rs                          # Public API re-exports
├── tree_lib.rs                     # Structure of the binary tree and DFS traversal
├── huffman_utils.rs                # Huffman tree, generating canonical codes, tree building
└── zip_lib.rs                      # Compression / decompression logic
tests/
└── tests.rs                        # Integration tests
```

### Example
```bash
$ echo "Hello, world! This is a test." > input.txt
$ cargo run zip input.txt compressed.bin
File was zipped correctly.
$ cargo run unzip compressed.bin restored.txt
File was unzipped correctly.
$ diff input.txt restored.txt   # no output, files are identical
```

## API
The library exposes two functions:

```rust
pub fn zip(input_filename: String, output_filename: String) -> Result<String, String>;
pub fn unzip(input_filename: String, output_filename: String) -> Result<String, String>;
```

Both return a success message or an error string describing the problem.

## Testing
Run the integration tests:
```bash
cargo test
```

Tests cover:
- Round-trip compression/decompression
- Empty files
- Single-byte files
- Uniform byte data
- All 256 bytes with equal frequency
- Corrupted archives (short file, damaged header, wrong size)

## Error Types
Error might be different types:
- **Problems with creating tree**
    - `Internal error: left/right child unexpectedly None`
- **Problems in work with file**
    - `Problem reading/creating the file`
    - `Failed to get metadata`
    - `Problem writing the file`
    - `Problem seeking file`
- **Problems with zipped file**
    - `Incorrect type of the zipped file`
    - `Decompressed size does not match expected size`
    - `The zipped file has the empty table`
- **Problem with empty file**
    - `File is empty, nothing to compress.`


## License
This project is licensed under the Apache-2.0 License – see the [LICENSE](LICENSE) file for details.

## Author
- [Iseyne](https://github.com/Iseyne)