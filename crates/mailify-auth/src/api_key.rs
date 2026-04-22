use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

#[derive(Debug, thiserror::Error)]
pub enum ApiKeyError {
    #[error("hash error: {0}")]
    Hash(String),
    #[error("verify error: {0}")]
    Verify(String),
}

pub fn hash_api_key(plaintext: &str) -> Result<String, ApiKeyError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(plaintext.as_bytes(), &salt)
        .map_err(|e| ApiKeyError::Hash(e.to_string()))?;
    Ok(hash.to_string())
}

pub fn verify_api_key(plaintext: &str, stored_hash: &str) -> Result<bool, ApiKeyError> {
    let parsed = PasswordHash::new(stored_hash).map_err(|e| ApiKeyError::Verify(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(plaintext.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_roundtrip() {
        let key = "super-secret-api-key";
        let hash = hash_api_key(key).unwrap();
        assert!(verify_api_key(key, &hash).unwrap());
        assert!(!verify_api_key("wrong", &hash).unwrap());
    }
}
