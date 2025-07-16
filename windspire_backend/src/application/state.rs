use crate::application::config::AppConfig;
use crate::application::services::{jwt_service::JwtService, oauth_service::OAuthService};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub jwt_service: Arc<JwtService>,
    pub oauth_service: Arc<OAuthService>,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(
        db_pool: PgPool,
        jwt_service: Arc<JwtService>,
        oauth_service: Arc<OAuthService>,
        config: AppConfig,
    ) -> Self {
        Self {
            db_pool,
            jwt_service,
            oauth_service,
            config,
        }
    }

    pub fn pool(&self) -> &PgPool {
        &self.db_pool
    }
}
