use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("jwt encode failed: {0}")]
    Encode(String),
    #[error("jwt decode failed: {0}")]
    Decode(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — API key identifier.
    pub sub: String,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
    /// Optional scopes (reserved for future granular perms).
    #[serde(default)]
    pub scopes: Vec<String>,
}

#[derive(Clone)]
pub struct JwtIssuer {
    secret: String,
    issuer: String,
    ttl_secs: i64,
}

impl JwtIssuer {
    pub fn new(secret: impl Into<String>, issuer: impl Into<String>, ttl_secs: u64) -> Self {
        Self {
            secret: secret.into(),
            issuer: issuer.into(),
            ttl_secs: ttl_secs as i64,
        }
    }

    pub fn issue(&self, subject: impl Into<String>, scopes: Vec<String>) -> Result<String, JwtError> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: subject.into(),
            iss: self.issuer.clone(),
            iat: now,
            exp: now + self.ttl_secs,
            scopes,
        };
        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_bytes()))
            .map_err(|e| JwtError::Encode(e.to_string()))
    }

    pub fn verify(&self, token: &str) -> Result<Claims, JwtError> {
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.issuer]);
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|e| JwtError::Decode(e.to_string()))?;
        Ok(data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_and_verify_roundtrip() {
        let issuer = JwtIssuer::new("secret-abc", "mailify", 60);
        let token = issuer.issue("client-42", vec!["mail:send".into()]).unwrap();
        let claims = issuer.verify(&token).unwrap();
        assert_eq!(claims.sub, "client-42");
        assert_eq!(claims.iss, "mailify");
        assert_eq!(claims.scopes, vec!["mail:send"]);
    }

    #[test]
    fn wrong_issuer_rejected() {
        let a = JwtIssuer::new("secret", "issuer-a", 60);
        let b = JwtIssuer::new("secret", "issuer-b", 60);
        let token = a.issue("sub", vec![]).unwrap();
        assert!(b.verify(&token).is_err());
    }

    #[test]
    fn wrong_secret_rejected() {
        let a = JwtIssuer::new("secret-1", "mailify", 60);
        let b = JwtIssuer::new("secret-2", "mailify", 60);
        let token = a.issue("sub", vec![]).unwrap();
        assert!(b.verify(&token).is_err());
    }
}
