use actix_web::{get, web, App, HttpServer, Responder};
use anyhow::{Context, Result};
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {

    format!("Hello {}", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().service(greet)
    })
        .bind(("127.0.0.1", 8989))?.run().await

}
