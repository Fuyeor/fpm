// src/utils/token.rs
use rand::RngExt;
use rand::distr::Alphanumeric;
use sha2::{Digest, Sha256};

/// Returns (plain_token, token_hash)
/// The plain_token should be shown to the user EXACTLY ONCE.
/// The token_hash must be saved in the database.
pub fn generate_fpm_token() -> (String, String) {
    // Generate 32 bytes of secure randomness
    let random_str: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let plain_token = format!("fpm_{}", random_str);
    let token_hash = hash_token(&plain_token);

    (plain_token, token_hash)
}

/// Computes SHA-256 hash of a string (Fail-fast standard)
pub fn hash_token(plain_token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(plain_token.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
