use crate::util;

use base64;
use sha2::{Digest, Sha256};

pub struct PkceToken {
    pub code_verifier: String,
    pub code_challenge: String,
}

fn base64_encode_string(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    base64::encode_config(hasher.finalize(), base64::URL_SAFE)
}

impl PkceToken {
    pub fn new() -> Self {
        let code_verifier = util::generate_random_string(128);
        let code_challenge = base64_encode_string(&code_verifier);
        PkceToken {
            code_verifier,
            code_challenge,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.code_challenge == base64_encode_string(&self.code_verifier)
    }
}
