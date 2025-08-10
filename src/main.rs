use anyhow::{Context, Result, anyhow};
use clap::{Parser, ValueEnum};

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
}
