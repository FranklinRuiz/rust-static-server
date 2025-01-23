use actix_web::{web, App, HttpServer};
use actix_web::middleware::Compress;
use crate::infrastructure::server_handler::{index_handler, static_handler};

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .route("/", web::get().to(index_handler))
            .route("/{filename:.*}", web::get().to(static_handler))
            .default_service(web::route().to(index_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}