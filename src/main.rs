mod handlers;

use handlers::server::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting server on http://127.0.0.1:8080...");

    if let Err(e) = start_server().await {
        eprintln!("❌ Server failed to start: {}", e);
    }

    Ok(())
}
