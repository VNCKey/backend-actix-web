//! Gestion de recursos
//! Build connection string e inicializar el PgPool
//!
//! Si al final requiero cambiar a otra BD aqui solo se modifica el codigo para no
//! afectar el main.rs

use crate::infrastructure::config;
use sqlx::{Error as SqlxError, PgPool};

/// Estructura para los errores que pueda ocurrir durante la inicializacion

#[derive(Debug)]
pub enum DataBaseError {
    ConnectionError(SqlxError),
    InvalidConfig,
}

/// Inicializa el pool de conexiones de PostgreSQL.
pub async fn setup_db_pool(cfg: &config::Settings) -> Result<PgPool, DataBaseError> {
    // 1. Determinar la cadena de conexión (Connection String)
    let conn_str = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Fallback: Construir a partir de los campos del TOML.
        format!(
            "postgres://{}:{}@{}:{}/{}",
            cfg.database.user,
            cfg.database.password,
            cfg.database.host,
            cfg.database.port,
            cfg.database.name,
        )
    });

    // Pequeña mejora: si usamos TOML y la string construida está vacía, es un error.
    if conn_str.is_empty() {
        return Err(DataBaseError::InvalidConfig);
    }

    println!("Conectando a PostgreSQL...");

    // 2. Crear pool y mapear el error de sqlx al error de nuestro módulo.
    PgPool::connect(&conn_str)
        .await
        .map_err(DataBaseError::ConnectionError)
}

// Puedes añadir aquí una función para ejecutar las migraciones si quieres.
