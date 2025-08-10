use anyhow::{Context, Result, anyhow};
use clap::{Parser, ValueEnum};
use sha2::{Digest, Sha256};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    pattern: String,

    #[arg(long, default_value_t = HashingProtocol::Sha256)]
    protocol: HashingProtocol,

    #[arg(long, default_value_t = 6)]
    input_length: usize,
}

#[derive(ValueEnum, Clone)]
enum HashingProtocol {
    Sha256,
    Md5,
}

impl std::fmt::Display for HashingProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                HashingProtocol::Sha256 => "sha256",
                HashingProtocol::Md5 => "md5",
            }
        )
    }
}

fn validate_hex_pattern(s: &str, max_length: usize) -> Result<String, String> {
    if s.is_empty() {
        return Err(String::from("Pattern cannot be empty."));
    }

    if s.len() > max_length {
        return Err("Pattern cannot exceed the length of the hash digest.".to_string());
    }

    if !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Pattern must only contain hexadecimal digits.".into());
    }

    Ok(s.to_lowercase())
}

struct MinedMatch {
    input: String,
    digest: String,
}

fn mine(
    pattern: &str,
    _protocol: &HashingProtocol,
    max_input_length: usize,
) -> Result<Option<MinedMatch>> {
    let max_input_number: usize = 10_u64.pow(max_input_length as u32) as usize;

    let pattern_bytes: Vec<u8> =
        hex::decode(pattern).context("Decoding hex-encoded string to Vec<u8>")?;

    for i in 0..=max_input_number {
        let s = i.to_string();
        match compare_patterns(&s, &pattern_bytes) {
            Ok(o) => match o {
                None => continue,
                Some(digest) => {
                    return Ok(Some(MinedMatch {
                        input: i.to_string(),
                        digest,
                    }));
                }
            },
            Err(e) => return Err(anyhow!(e)),
        };
    }

    Ok(None)
}

fn compare_patterns(input: &str, pattern: &Vec<u8>) -> Result<Option<String>> {
    let digest: [u8; 32] = Sha256::digest(input.as_bytes()).into();

    if &digest[..pattern.len()] == pattern.as_slice() {
        return Ok(Some(hex::encode(digest)));
    }

    Ok(None)
}

fn main() {
    let args: Args = Args::parse();

    let pattern: String = match validate_hex_pattern(&args.pattern, 64) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    println!(
        "Pattern to match: {}\nHashing protocol: {}\nMax input length: {}",
        pattern, args.protocol, args.input_length
    );

    match mine(&pattern, &args.protocol, args.input_length) {
        Ok(opt) => match opt {
            Some(m) => println!(
                "Matching Pattern Found\nInput: {}, Digest: {}",
                m.input, m.digest
            ),
            None => println!("No matching patterns found for pattern: {}", pattern),
        },
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
