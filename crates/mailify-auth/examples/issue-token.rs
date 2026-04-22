//! Mint a JWT offline using the same secret as the running server.
//!
//! Reads: MAILIFY_AUTH__JWT_SECRET (required),
//!        MAILIFY_AUTH__JWT_ISSUER (default: mailify),
//!        MAILIFY_AUTH__JWT_TTL_SECS (default: 3600)
//!
//! Usage: cargo run -p mailify-auth --example issue-token -- <subject> [scope1,scope2,...]

use mailify_auth::JwtIssuer;

fn main() {
    // Load .env/.env.local so local dev just works.
    let _ = dotenvy::dotenv();
    let _ = dotenvy::from_filename(".env.local");

    let mut args = std::env::args().skip(1);
    let subject = args.next().unwrap_or_else(|| "dev".to_string());
    let scopes: Vec<String> = args
        .next()
        .map(|s| {
            s.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    let secret = std::env::var("MAILIFY_AUTH__JWT_SECRET").unwrap_or_else(|_| {
        eprintln!(
            "WARN: MAILIFY_AUTH__JWT_SECRET not set — using built-in default 'CHANGE_ME_IN_PRODUCTION'.\n       The server will only accept this token if it runs with the same default."
        );
        "CHANGE_ME_IN_PRODUCTION".to_string()
    });
    let issuer_name =
        std::env::var("MAILIFY_AUTH__JWT_ISSUER").unwrap_or_else(|_| "mailify".to_string());
    let ttl = std::env::var("MAILIFY_AUTH__JWT_TTL_SECS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(3600);

    let issuer = JwtIssuer::new(secret, issuer_name, ttl);
    let token = issuer.issue(subject, scopes).expect("jwt issue");
    println!("{token}");
}
