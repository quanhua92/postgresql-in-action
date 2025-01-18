use config::Config;
use secrecy::{ExposeSecret, SecretString};

#[allow(dead_code)]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppConfig {
    pub(crate) app_port: i32,
    pub(crate) db_host: String,
    pub(crate) db_port: i32,
    pub(crate) db_name: String,
    pub(crate) db_username: String,
    pub(crate) db_password: SecretString,
}

impl AppConfig {
    pub fn load() -> Result<Self, anyhow::Error> {
        let mut builder = Config::builder();
        builder = builder
            .set_default("APP_PORT", 3000)
            .expect("Failed to set default APP_PORT");
        let reader = builder.add_source(config::Environment::default()).build()?;
        Ok(reader.try_deserialize()?)
    }

    pub fn app_port(&self) -> i32 {
        self.app_port
    }

    pub fn db_host(&self) -> &str {
        &self.db_host
    }
    pub fn db_name(&self) -> &str {
        &self.db_name
    }
    pub fn db_username(&self) -> &str {
        &self.db_username
    }
    pub fn db_password(&self) -> &str {
        &self.db_password.expose_secret()
    }
    pub fn set_db_port(&mut self, db_port: i32) {
        self.db_port = db_port;
    }

    pub fn database_url(&self) -> SecretString {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.db_username,
            self.db_password.expose_secret().to_string(),
            self.db_host,
            self.db_port,
            self.db_name
        )
        .into()
    }

    pub fn database_url_without_db_name(&self) -> SecretString {
        format!(
            "postgresql://{}:{}@{}:{}",
            self.db_username,
            self.db_password.expose_secret().to_string(),
            self.db_host,
            self.db_port,
        )
        .into()
    }
}
