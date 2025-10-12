use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub enum FirebaseError {
    TokenValidation(String),
    HttpError(reqwest::Error),
    InvalidToken,
    ProjectIdMismatch,
    TokenExpired,
    InvalidIssuer,
    InvalidAudience,
}

impl std::fmt::Display for FirebaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirebaseError::TokenValidation(msg) => write!(f, "Token validation error: {}", msg),
            FirebaseError::HttpError(e) => write!(f, "HTTP error: {}", e),
            FirebaseError::InvalidToken => write!(f, "Invalid token"),
            FirebaseError::ProjectIdMismatch => write!(f, "Project ID mismatch"),
            FirebaseError::TokenExpired => write!(f, "Token has expired"),
            FirebaseError::InvalidIssuer => write!(f, "Invalid token issuer"),
            FirebaseError::InvalidAudience => write!(f, "Invalid token audience"),
        }
    }
}

impl std::error::Error for FirebaseError {}

impl From<reqwest::Error> for FirebaseError {
    fn from(err: reqwest::Error) -> Self {
        FirebaseError::HttpError(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirebaseTokenClaims {
    pub iss: String,
    pub aud: String,
    pub auth_time: i64,
    pub user_id: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub firebase: Option<FirebaseInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirebaseInfo {
    pub identities: Option<HashMap<String, Vec<String>>>,
    pub sign_in_provider: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirebaseUserInfo {
    pub uid: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email_verified: bool,
    pub provider: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirebasePublicKey {
    pub kty: String,
    pub alg: String,
    pub r#use: String,
    pub kid: String,
    pub n: String,
    pub e: String,
}

pub struct FirebaseService {
    project_id: String,
    client: Client,
}

impl FirebaseService {
    pub fn new(project_id: String) -> Self {
        Self {
            project_id,
            client: Client::new(),
        }
    }

    /// Verify Firebase ID token and extract user information
    pub async fn verify_id_token(&self, id_token: &str) -> Result<FirebaseUserInfo, FirebaseError> {
        // Decode token header to get key ID (kid)
        let header = decode_header(id_token).map_err(|e| {
            FirebaseError::TokenValidation(format!("Failed to decode header: {}", e))
        })?;

        let kid = header.kid.ok_or(FirebaseError::InvalidToken)?;

        // Get public keys from Google
        let public_keys = self.get_firebase_public_keys().await?;
        let public_key = public_keys.get(&kid).ok_or(FirebaseError::TokenValidation(
            "Key ID not found".to_string(),
        ))?;

        // Create RSA decoding key from the public key
        let decoding_key = self.create_decoding_key(public_key)?;

        // Set up validation
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[format!(
            "https://securetoken.google.com/{}",
            self.project_id
        )]);
        validation.set_audience(std::slice::from_ref(&self.project_id));

        // Decode and validate token
        let token_data = decode::<FirebaseTokenClaims>(id_token, &decoding_key, &validation)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => FirebaseError::TokenExpired,
                jsonwebtoken::errors::ErrorKind::InvalidIssuer => FirebaseError::InvalidIssuer,
                jsonwebtoken::errors::ErrorKind::InvalidAudience => FirebaseError::InvalidAudience,
                _ => FirebaseError::TokenValidation(format!("Token validation failed: {}", e)),
            })?;

        // Convert to our FirebaseUserInfo struct
        let user_info = FirebaseUserInfo {
            uid: token_data.claims.user_id,
            email: token_data.claims.email,
            name: token_data.claims.name,
            picture: token_data.claims.picture,
            email_verified: token_data.claims.email_verified.unwrap_or(false),
            provider: token_data.claims.firebase.and_then(|f| f.sign_in_provider),
        };

        Ok(user_info)
    }

    /// Fetch Firebase public keys from Google
    async fn get_firebase_public_keys(
        &self,
    ) -> Result<HashMap<String, FirebasePublicKey>, FirebaseError> {
        let url = "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(FirebaseError::TokenValidation(format!(
                "Failed to fetch public keys: HTTP {}",
                response.status()
            )));
        }

        // The response is a JSON object with certificate strings
        let _certs: HashMap<String, String> = response.json().await.map_err(|e| {
            FirebaseError::TokenValidation(format!("Failed to parse certificates: {}", e))
        })?;

        // For this implementation, we'll use a simplified approach
        // In production, you'd want to parse the X.509 certificates properly
        // For now, we'll return an error indicating this needs proper implementation
        Err(FirebaseError::TokenValidation(
            "Certificate parsing not yet implemented. Use Firebase Admin SDK or implement X.509 certificate parsing.".to_string()
        ))
    }

    /// Create RSA decoding key from Firebase public key
    fn create_decoding_key(
        &self,
        _public_key: &FirebasePublicKey,
    ) -> Result<DecodingKey, FirebaseError> {
        // This is a placeholder - in a real implementation you'd convert the
        // X.509 certificate to an RSA public key
        Err(FirebaseError::TokenValidation(
            "RSA key creation not yet implemented".to_string(),
        ))
    }

    /// Simple token validation without cryptographic verification (for development/testing)
    /// WARNING: This should NOT be used in production!
    #[cfg(debug_assertions)]
    pub async fn verify_id_token_unsafe(
        &self,
        id_token: &str,
    ) -> Result<FirebaseUserInfo, FirebaseError> {
        // Decode without verification (UNSAFE - only for development)
        let mut validation = Validation::new(Algorithm::RS256);
        validation.insecure_disable_signature_validation();
        validation.set_issuer(&[format!(
            "https://securetoken.google.com/{}",
            self.project_id
        )]);
        validation.set_audience(std::slice::from_ref(&self.project_id));

        let token_data = decode::<FirebaseTokenClaims>(
            id_token,
            &DecodingKey::from_secret(&[]), // Empty key since we're not validating
            &validation,
        )
        .map_err(|e| FirebaseError::TokenValidation(format!("Token parsing failed: {}", e)))?;

        let user_info = FirebaseUserInfo {
            uid: token_data.claims.user_id,
            email: token_data.claims.email,
            name: token_data.claims.name,
            picture: token_data.claims.picture,
            email_verified: token_data.claims.email_verified.unwrap_or(false),
            provider: token_data.claims.firebase.and_then(|f| f.sign_in_provider),
        };

        Ok(user_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firebase_service_creation() {
        let service = FirebaseService::new("test-project".to_string());
        assert_eq!(service.project_id, "test-project");
    }

    // Add more tests here for token validation logic
}
