mod config;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use sqlx::PgPool;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Cargar config
    let cfg = config::get_config();

    // 2. Construir connection string
    let conn_str = format!(
        "postgres://{}:{}@{}:{}/{}",
        cfg.database.user,
        cfg.database.password,
        cfg.database.host,
        cfg.database.port,
        cfg.database.name,
    );
    println!("Iniciando servidor...");
    println!("Conectando a DB: {}", conn_str);
    // 3. Crear pool de PostgreSQL
    let pool = PgPool::connect(&conn_str)
        .await
        .expect("No se pudo conectar a PostgreSQL");

    // 4. Servidor Actix
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
