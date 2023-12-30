use actix_files as fs;
use actix_web::{App, HttpServer};
mod routes;

use routes::todo_routes::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(fs::Files::new("/", "frontend/dist").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}