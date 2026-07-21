use std::{time::{Duration, SystemTime, UNIX_EPOCH}};

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result, get, http::{Error, header::ContentType}, route, rt, web,
};
use actix_web_rust_embed_responder::IntoResponse;
use actix_ws::AggregatedMessage;
use rust_embed_for_web::RustEmbed;
use serde::Serialize;
use tokio::time::interval;
use futures_util::StreamExt as _;

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

#[derive(Serialize)]
struct StdOutData {
    timestamp: u64,
    data: String,
}

async fn ws_stream(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream).unwrap();

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        let mut timer = interval(Duration::from_secs(1));
        let mut count = 0;

        loop {
            tokio::select! {
                _ = timer.tick() => {
                    count += 1;
                    let payload = StdOutData {
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        data: format!("{}", count),
                    };

                    let json = serde_json::to_string(&payload).unwrap();
                    if session.text(json).await.is_err() {
                        break;
                    }
                }
                msg = stream.next() => {
                    match msg {
                        Some(Ok(AggregatedMessage::Close(_))) | None => break,
                        Some(Ok(AggregatedMessage::Ping(bytes))) => {
                            let _ = session.pong(&bytes).await;
                        }
                        _ => (),
                    }
                }
            }
        }
    });

    Ok(res)
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
            .route("/api/stream", web::get().to(ws_stream))
            .service(serve_assets)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
