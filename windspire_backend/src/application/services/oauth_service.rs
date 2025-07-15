use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::config::OAuthConfig;
use crate::domain::models::auth::{AuthError, GoogleUserInfo};

#[derive(Debug)]
pub enum OAuthError {
    ConfigurationError(String),
    TokenExchangeError(String),
    UserInfoError(String),
    HttpError(reqwest::Error),
    InvalidState,
}

impl std::fmt::Display for OAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthError::ConfigurationError(msg) => write!(f, "OAuth configuration error: {}", msg),
            OAuthError::TokenExchangeError(msg) => write!(f, "Token exchange error: {}", msg),
            OAuthError::UserInfoError(msg) => write!(f, "User info error: {}", msg),
            OAuthError::HttpError(e) => write!(f, "HTTP error: {}", e),
            OAuthError::InvalidState => write!(f, "Invalid CSRF state"),
        }
    }
}

impl std::error::Error for OAuthError {}

impl From<reqwest::Error> for OAuthError {
    fn from(error: reqwest::Error) -> Self {
        OAuthError::HttpError(error)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthorizationUrl {
    pub url: String,
    pub state: String,
}

pub struct OAuthService {
    client: BasicClient,
    http_client: Client,
    config: OAuthConfig,
}

impl OAuthService {
    pub fn new(config: OAuthConfig) -> Result<Self, OAuthError> {
        let client_id = ClientId::new(config.google_client_id.clone());
        let client_secret = ClientSecret::new(config.google_client_secret.clone());

        let auth_url = AuthUrl::new(config.google_auth_url.clone())
            .map_err(|e| OAuthError::ConfigurationError(format!("Invalid auth URL: {}", e)))?;

        let token_url = TokenUrl::new(config.google_token_url.clone())
            .map_err(|e| OAuthError::ConfigurationError(format!("Invalid token URL: {}", e)))?;

        let redirect_url = RedirectUrl::new(config.google_redirect_uri.clone())
            .map_err(|e| OAuthError::ConfigurationError(format!("Invalid redirect URI: {}", e)))?;

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);

        let http_client = Client::new();

        Ok(Self {
            client,
            http_client,
            config,
        })
    }

    pub fn get_authorization_url(&self) -> AuthorizationUrl {
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        AuthorizationUrl {
            url: auth_url.to_string(),
            state: csrf_token.secret().clone(),
        }
    }

    pub async fn exchange_code_for_token(
        &self,
        code: &str,
        state: &str,
        expected_state: &str,
    ) -> Result<String, OAuthError> {
        // Verify CSRF state
        if state != expected_state {
            return Err(OAuthError::InvalidState);
        }

        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|e| OAuthError::TokenExchangeError(format!("Token exchange failed: {}", e)))?;

        Ok(token_result.access_token().secret().clone())
    }

    pub async fn get_user_info(&self, access_token: &str) -> Result<GoogleUserInfo, OAuthError> {
        let response = self
            .http_client
            .get(&self.config.google_userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(OAuthError::UserInfoError(format!(
                "Failed to get user info: HTTP {}",
                response.status()
            )));
        }

        let user_info: GoogleUserInfo = response
            .json()
            .await
            .map_err(|e| OAuthError::UserInfoError(format!("Failed to parse user info: {}", e)))?;

        Ok(user_info)
    }

    pub fn generate_state() -> String {
        Uuid::new_v4().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> OAuthConfig {
        OAuthConfig {
            google_client_id: "test-client-id".to_string(),
            google_client_secret: "test-client-secret".to_string(),
            google_redirect_uri: "http://localhost:8080/auth/callback".to_string(),
            google_auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            google_token_url: "https://oauth2.googleapis.com/token".to_string(),
            google_userinfo_url: "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
        }
    }

    #[test]
    fn test_oauth_service_creation() {
        let config = create_test_config();
        let service = OAuthService::new(config);
        assert!(service.is_ok());
    }

    #[test]
    fn test_get_authorization_url() {
        let config = create_test_config();
        let service = OAuthService::new(config).unwrap();
        let auth_url = service.get_authorization_url();

        assert!(auth_url.url.contains("accounts.google.com"));
        assert!(auth_url.url.contains("client_id=test-client-id"));
        assert!(!auth_url.state.is_empty());
    }

    #[test]
    fn test_generate_state() {
        let state1 = OAuthService::generate_state();
        let state2 = OAuthService::generate_state();

        assert_ne!(state1, state2);
        assert!(!state1.is_empty());
        assert!(!state2.is_empty());
    }
}
