use crate::config::Settings;
use crate::db::postgres::init_db;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn new(settings: &Settings) -> Self {
        let db = init_db(
            &settings.database_url
        ).await;
        AppState { db }
    }
}