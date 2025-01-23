use actix_files as fs;
use actix_web::{
    middleware::{Compress, DefaultHeaders, Logger},
    App, HttpServer,
};

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(DefaultHeaders::new().add(("Cache-Control", "public, max-age=86400")))
            .wrap(Logger::new("%a %r %s %b %{User-Agent}i %T"))
            .service(
                fs::Files::new("/", "./static")
                    .index_file("index.html")
                    .use_last_modified(true)
                    .prefer_utf8(true),
            )
    })
    .workers(num_cpus::get())
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
