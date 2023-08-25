use serde::*;

use crate::util;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Scope {
    Read,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TokenType {
    Bearer,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessToken {
    access_token: String,
    expires_in: i32,
    scope: Scope,
    token_type: TokenType,
}

const TOKEN_LENGTH: usize = 32;

// Default expiry is 24 hours
const DEFAULT_TOKEN_EXPIRY_PERIOD: i32 = 60 * 60 * 24;

fn generate_random_token() -> String {
    util::generate_random_string(TOKEN_LENGTH)
}

impl AccessToken {
    pub fn new(scope: Scope, token_type: TokenType) -> Self {
        AccessToken {
            access_token: generate_random_token(),
            expires_in: DEFAULT_TOKEN_EXPIRY_PERIOD,
            scope,
            token_type,
        }
    }
}
