mod application;
mod infrastructure;

use application::server::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    start_server().await
}