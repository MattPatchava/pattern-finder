# pattern-finder

A high-performance CLI tool to search for inputs matching a hex hash prefix.

## Features
- Parallelised brute-force search using Rayon.
- Pattern validation to ensure only valid hex digits are used.
- Modular mining logic for easier extension and optimisation.
- Uses [clap](https://crates.io/crates/clap) for CLI argument parsing.
- Flexible hashing protocol enum for future algorithm support.

## Performance
By leveraging [Rayon](https://crates.io/crates/rayon), the mining loop is fully parallelised across all CPU cores.
On a 10-core system, the miner utilises 1000% CPU and drastically reduces search time compared to single-threaded brute force.

## Usage

```bash
cargo run -- --pattern <HEX_PATTERN> [--protocol sha256|md5] [--input-length N]
```
