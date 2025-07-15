use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_address: String,
    pub oauth: OAuthConfig,
    pub jwt: JwtConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub google_auth_url: String,
    pub google_token_url: String,
    pub google_userinfo_url: String,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
    pub issuer: String,
}

#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(AppConfig {
            database_url: env::var("DATABASE_URL")?,
            server_address: env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_string()),
            oauth: OAuthConfig {
                google_client_id: env::var("GOOGLE_CLIENT_ID")?,
                google_client_secret: env::var("GOOGLE_CLIENT_SECRET")?,
                google_redirect_uri: env::var("GOOGLE_REDIRECT_URI")
                    .unwrap_or("http://localhost:8080/auth/callback".to_string()),
                google_auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                google_token_url: "https://oauth2.googleapis.com/token".to_string(),
                google_userinfo_url: "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET")?,
                expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                    .unwrap_or("24".to_string())
                    .parse()
                    .unwrap_or(24),
                issuer: env::var("JWT_ISSUER").unwrap_or("windspire".to_string()),
            },
            cors: CorsConfig {
                allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                    .unwrap_or("http://localhost:3000,http://localhost:5173".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                    "OPTIONS".to_string(),
                ],
                allowed_headers: vec![
                    "Content-Type".to_string(),
                    "Authorization".to_string(),
                    "Accept".to_string(),
                ],
            },
        })
    }
}
