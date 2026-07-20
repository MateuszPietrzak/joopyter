use actix_web::{
    App, HttpResponse, HttpServer, Responder, Result, get, http::header::ContentType, route, web,
};
use actix_web_rust_embed_responder::IntoResponse;
use rust_embed_for_web::RustEmbed;
use serde::Serialize;

#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Frontend;

#[route("/{path:.*}", method = "GET", method = "HEAD")]
async fn serve_assets(path: web::Path<String>) -> impl Responder {
    let path = if path.is_empty() {
        "index.html"
    } else {
        path.as_str()
    };
    Frontend::get(path).into_response()
}
#[derive(Clone, PartialEq, Serialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[get("/api/ping")]
async fn ping() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Pong"))
}

#[get("/api/data")]
async fn data() -> HttpResponse {
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".into(),
            speaker: "John Doe".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
        Video {
            id: 2,
            title: "The development process".into(),
            speaker: "Jane Smith".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".into(),
            speaker: "Matt Miller".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
        Video {
            id: 4,
            title: "Mouseless development".into(),
            speaker: "Tom Jerry".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
    ];
    HttpResponse::Ok().json(videos)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(data)
            .service(serve_assets)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
