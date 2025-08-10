use anyhow::{anyhow, Context, Result};
use sha2::{Digest, Sha256};

use crate::HashingProtocol;

pub struct MinedMatch {
    input: String,
    digest: String,
}

impl MinedMatch {
    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

pub fn mine(
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
