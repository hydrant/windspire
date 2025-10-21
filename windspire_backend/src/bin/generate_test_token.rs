use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    name: String,
    picture: Option<String>,
    roles: Vec<String>,
    permissions: Vec<String>,
    iat: i64,
    exp: i64,
}

fn main() {
    let secret = "your-super-secret-jwt-key-change-this-in-production";
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    let now = Utc::now();
    let expiration = now + Duration::hours(24);

    // Use one of the test users from the migration
    let user_id = "01964081-4fbf-747a-ae64-d17030fc3dcc"; // Ove Størholt

    let claims = Claims {
        sub: user_id.to_string(),
        email: "ovestoerholt@gmail.com".to_string(),
        name: "Ove Størholt".to_string(),
        picture: None,
        roles: vec!["admin".to_string()],
        permissions: vec!["admin:write".to_string(), "boats:read".to_string()],
        iat: now.timestamp(),
        exp: expiration.timestamp(),
    };

    let header = Header::new(Algorithm::HS256);

    match encode(&header, &claims, &encoding_key) {
        Ok(token) => println!("{}", token),
        Err(e) => eprintln!("Error generating token: {}", e),
    }
}
