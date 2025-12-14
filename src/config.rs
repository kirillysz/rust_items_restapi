use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub database_url: String
}

impl Settings {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        envy::from_env().expect("Failed to read env variables")
    }
}
