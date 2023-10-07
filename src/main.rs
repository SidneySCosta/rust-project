use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use futures_util::stream::StreamExt;
use std::env;
use std::time::Instant;
use uuid::Uuid;

async fn upload(mut payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let mut bytes = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        bytes.extend_from_slice(&chunk?);
    }

    let start = Instant::now();
    let filepath = format!("uploads/{}", Uuid::new_v4());
    tokio::fs::write(&filepath, bytes).await?;
    let duration = start.elapsed();

    Ok(HttpResponse::Ok().body(format!(
        "Arquivo salvo em: {}\nTempo: {:?}",
        filepath, duration
    )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8081".to_string());

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload))
            .service(fs::Files::new("/download", "uploads/"))
    })
    .bind(&bind_address)?
    .run()
    .await
}
