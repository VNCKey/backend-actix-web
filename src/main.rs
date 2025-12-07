// Nuestro main.rs esta listo para Inyección de dependencias por Constructor "DI"
// Orquestacion
// Inicializar componentes y ensamblar la aplicación
// Me enfoco en solo iniciar la app en main.rs
// Y detalles como la base de datos o cualquier otro lo delego a otros archivos

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod infrastructure;

use infrastructure::config;
use infrastructure::db;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("VNCKey")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Cargar la configuración
    let cfg = config::get_config();

    // 2. Inicializar la Base de Datos (Lógica en infrastructure::db)
    let pool = match db::setup_db_pool(&cfg).await {
        // Manejo de errores
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Error al inicializar la base de datos: {:?}", err);
            std::process::exit(1);
        }
    };

    let bind_addr = cfg.server.bind.unwrap_or("0.0.0.0".into());
    let port = cfg.server.port.unwrap_or(8080);

    println!("Iniciando servidor en {}:{}", bind_addr, port);

    // 3. Iniciar el Servidor Actix
    HttpServer::new(move || {
        App::new()
            // Inyectamos el pool de DB para que esté disponible en los Handlers
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health))
    })
    .bind((bind_addr.as_str(), port))?
    .run()
    .await
}
