use sea_orm::Database;
use sea_orm::DatabaseConnection;

pub async fn init_db(database_url: &str) -> DatabaseConnection {
    Database::connect(database_url)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Database URL: {}", database_url);
            panic!("Failed to connect to database: {:?}", err)
        })
}