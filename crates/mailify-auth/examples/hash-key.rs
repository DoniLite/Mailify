//! Generate an argon2 hash for an API key and print the env var line to paste into `.env`.
//!
//! Usage: cargo run -p mailify-auth --example hash-key -- <plaintext> [id]

use mailify_auth::hash_api_key;

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(plaintext) = args.next() else {
        eprintln!("usage: hash-key <plaintext> [id]");
        std::process::exit(2);
    };
    let id = args
        .next()
        .unwrap_or_else(|| "key".to_string())
        .to_uppercase();
    let hash = hash_api_key(&plaintext).expect("argon2 hash");
    println!("MAILIFY_AUTH__API_KEYS__{id}={hash}");
}
