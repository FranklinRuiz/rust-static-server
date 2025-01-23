use actix_web::{HttpRequest, Responder};
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir<'_> = include_dir!("static");

pub async fn index_handler(_req: HttpRequest) -> impl Responder {
    let file = STATIC_DIR
        .get_file("index.html")
        .expect("index.html not found");
    let contents = file.contents_utf8().expect("Failed to read index.html");
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(contents)
}

pub async fn static_handler(req: HttpRequest) -> impl Responder {
    let path = req.path().trim_start_matches('/');

    if path.is_empty() || path == "/" {
        return serve_index_file();
    }

    if let Some(file) = STATIC_DIR.get_file(path) {
        serve_static_file(file)
    } else {
        serve_index_file()
    }
}

fn serve_index_file() -> actix_web::HttpResponse {
    if let Some(index_file) = STATIC_DIR.get_file("index.html") {
        create_response("text/html; charset=utf-8", index_file.contents())
    } else {
        actix_web::HttpResponse::InternalServerError().body("index.html not found")
    }
}

fn serve_static_file(file: &include_dir::File) -> actix_web::HttpResponse {
    let mime = mime_guess::from_path(file.path()).first_or_text_plain();
    create_response(mime.as_ref(), file.contents())
}

fn create_response(content_type: &str, contents: &[u8]) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type(content_type)
        .body(contents.to_vec())
}
