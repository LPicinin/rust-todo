use actix_files as fs;
use actix_web::{App, HttpServer};
mod db;
mod routes;
mod handlers;
mod models;

use routes::todo_routes::config;

#[tokio::main]
fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(fs::Files::new("/", "frontend/dist").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}