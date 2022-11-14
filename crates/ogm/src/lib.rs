pub mod models;
pub mod services;
use aragog::{DatabaseConnection, Error, AuthMode};
const DEFAULT_DB_HOST: &str = "http://localhost:8529";
const DEFAULT_DB_NAME: &str = "aragog_test";
const DEFAULT_DB_USER: &str = "test";
const DEFAULT_DB_PASSWORD: &str = "test";

pub async fn set_up_database() -> Result<DatabaseConnection, Error> {
    DatabaseConnection::builder()
    .with_credentials(
        &std::env::var("DB_HOST").unwrap_or(DEFAULT_DB_HOST.to_owned()),
        &std::env::var("DB_NAME").unwrap_or(DEFAULT_DB_NAME.to_owned()),
        &std::env::var("DB_USER").unwrap_or(DEFAULT_DB_USER.to_owned()),
        &std::env::var("DB_PASSWORD").unwrap_or(DEFAULT_DB_PASSWORD.to_owned())
    )
    .with_auth_mode(AuthMode::Jwt)
    .apply_schema()
    .build()
    .await
}

pub mod prelude {
    pub use super::models::prelude::*;
    pub use super::services::prelude::*;
}