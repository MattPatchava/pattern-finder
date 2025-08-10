# pattern-finder

A CLI tool to search for inputs matching a hex hash prefix.

## Features
- Pattern validation to ensure only valid hex digits are used.
- Modular mining logic for easier extension and optimisation.
- Uses [clap](https://crates.io/crates/clap) for CLI argument parsing.
- Flexible hashing protocol enum for future algorithm support.

## Usage

```bash
cargo run -- --pattern <HEX_PATTERN> [--protocol sha256|md5] [--input-length N]
```
