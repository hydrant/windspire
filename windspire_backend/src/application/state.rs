use crate::application::config::AppConfig;
use crate::application::services::{jwt_service::JwtService, firebase_service::FirebaseService};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub jwt_service: Arc<JwtService>,
    pub firebase_service: Arc<FirebaseService>,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(
        db_pool: PgPool,
        jwt_service: Arc<JwtService>,
        firebase_service: Arc<FirebaseService>,
        config: AppConfig,
    ) -> Self {
        Self {
            db_pool,
            jwt_service,
            firebase_service,
            config,
        }
    }

    pub fn pool(&self) -> &PgPool {
        &self.db_pool
    }
}
