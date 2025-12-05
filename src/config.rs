use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
}

pub fn get_config() -> AppConfig {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()
        .unwrap();

    settings.try_deserialize::<AppConfig>().unwrap()
}
