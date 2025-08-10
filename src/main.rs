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

fn main() {
    let args: Args = Args::parse();

    println!(
        "Pattern to match: {}\nHashing protocol: {}\nMax input length: {}",
        args.pattern, args.protocol, args.input_length
    );
}
