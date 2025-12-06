mod config;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Cargar configuración
    let cfg = config::get_config();

    let bind_addr = cfg.server.bind.unwrap_or("0.0.0.0".into());
    let port = cfg.server.port.unwrap_or(8080);

    // 2. Construir connection string
    let conn_str = format!(
        "postgres://{}:{}@{}:{}/{}",
        cfg.database.user,
        cfg.database.password,
        cfg.database.host,
        cfg.database.port,
        cfg.database.name,
    );

    println!("Iniciando servidor en {}:{}", bind_addr, port);
    println!("Conectando a PostgreSQL: {}", conn_str);

    // 3. Crear pool
    let pool = PgPool::connect(&conn_str)
        .await
        .expect("No se pudo conectar a PostgreSQL");

    // 4. Servidor Actix
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health))
    })
    .bind((bind_addr.as_str(), port))?
    .run()
    .await
}
