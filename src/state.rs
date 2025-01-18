use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub(crate) config: AppConfig,
    pub(crate) db: PgPool,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, anyhow::Error> {
        let db = PgPool::connect(config.database_url().expose_secret())
            .await
            .map_err(|e| {
                tracing::error!("config: {config:?} error: {e}");
                e
            })?;
        Ok(Self { config, db })
    }

    pub fn db(&self) -> &PgPool {
        &self.db
    }
}
