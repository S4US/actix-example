use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde_json::json;

// Simple health check endpoint
async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Hello from Actix!"
    })))
}

// Simple GET endpoint
async fn hello() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "Hello, World!"
    })))
}

// Simple POST endpoint
async fn echo(body: web::Json<serde_json::Value>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "echo": body.into_inner()
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health))
            .route("/hello", web::get().to(hello))
            .route("/echo", web::post().to(echo))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
