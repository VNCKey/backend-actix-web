// Aqui se lee y se mapea los valores de mi .toml y ademas las variables de entorno

use config as config_rs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub bind: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    #[allow(dead_code)]
    pub url: Option<String>,

    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

// Una unica funcion que se encarga de la configuracion, encapsulamiento de la complejidad de las 12 Factor Apps
pub fn get_config() -> Settings {
    // Leer el entorno: default | dev | prod
    let env = std::env::var("RUST_ENV").unwrap_or("default".into());

    let builder = config_rs::Config::builder()
        .add_source(config_rs::File::with_name("config/default"))
        .add_source(
            config_rs::File::with_name(&format!("config/{}", env)).required(false), // si no existe prod.toml, no explota
        )
        .add_source(
            config_rs::Environment::default()
                .prefix("DATABASE")
                .separator("__"),
        );

    let cfg = builder.build().expect("Error cargando configuración");
    cfg.try_deserialize().expect("Config inválida")
}
