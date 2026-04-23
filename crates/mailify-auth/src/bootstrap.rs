//! First-boot bootstrap: when no API key is configured, generate an ephemeral one
//! and print the env line the operator needs to copy back into their environment.

use rand_core::{OsRng, RngCore};

use crate::api_key::{hash_api_key, ApiKeyError};

/// Result of a bootstrap run.
pub struct BootstrapKey {
    pub id: String,
    pub plaintext: String,
    pub hash: String,
}

/// Generate a random url-safe plaintext key and return it alongside its argon2 hash.
pub fn generate_bootstrap_key(id: impl Into<String>) -> Result<BootstrapKey, ApiKeyError> {
    let plaintext = random_token(32);
    let hash = hash_api_key(&plaintext)?;
    Ok(BootstrapKey {
        id: id.into(),
        plaintext,
        hash,
    })
}

/// Generate a random JWT secret (used when the default placeholder is still in place).
pub fn generate_jwt_secret() -> String {
    random_token(48)
}

fn random_token(byte_len: usize) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut bytes = vec![0u8; byte_len];
    OsRng.fill_bytes(&mut bytes);
    bytes
        .into_iter()
        .map(|b| ALPHABET[(b as usize) % ALPHABET.len()] as char)
        .collect()
}

/// Print a human-readable banner to stderr so it survives JSON log mode.
pub fn print_bootstrap_banner(key: &BootstrapKey, jwt_secret_generated: Option<&str>) {
    let mut lines = vec![
        "".to_string(),
        "===================== MAILIFY BOOTSTRAP =====================".to_string(),
        "No API key was configured. An ephemeral key has been generated".to_string(),
        "for this session. It will NOT survive a restart unless you add".to_string(),
        "the hash below to your environment (e.g. .env / compose env).".to_string(),
        "".to_string(),
        format!("  Key id:    {}", key.id),
        format!("  Plaintext: {}", key.plaintext),
        "".to_string(),
        "  Paste into your env and restart to persist:".to_string(),
        format!("    MAILIFY_AUTH__API_KEYS__{}={}", key.id, key.hash),
    ];
    if let Some(secret) = jwt_secret_generated {
        lines.push("".to_string());
        lines
            .push("  JWT secret was the default placeholder — generated a random one.".to_string());
        lines.push("  Persist it too (otherwise issued tokens die on restart):".to_string());
        lines.push(format!("    MAILIFY_AUTH__JWT_SECRET={secret}"));
    }
    lines.push("=============================================================".to_string());
    lines.push("".to_string());
    eprintln!("{}", lines.join("\n"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_key::verify_api_key;

    #[test]
    fn bootstrap_key_verifies_against_its_hash() {
        let key = generate_bootstrap_key("DEFAULT").unwrap();
        assert_eq!(key.id, "DEFAULT");
        assert!(verify_api_key(&key.plaintext, &key.hash).unwrap());
    }

    #[test]
    fn random_tokens_are_unique_and_sized() {
        let a = random_token(32);
        let b = random_token(32);
        assert_eq!(a.len(), 32);
        assert_ne!(a, b);
    }

    #[test]
    fn jwt_secret_is_non_empty() {
        let s = generate_jwt_secret();
        assert_eq!(s.len(), 48);
    }
}
