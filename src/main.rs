#[macro_use]
extern crate bson;

use envfile::EnvFile;
use std::path::Path;
use actix_web::{HttpServer, App, middleware, web};
use actix_web::http::ContentEncoding;

mod config;
mod db;
mod repository;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
    App.new()
    .wrap(middleware::Compress::new(ContentEncoding::Br))
    .wrap(middleware::Logger::default())
    .service(
        web::scope("/user")
        .configure(repository::user_repository::init_routes)
)
)
.bind("127.0.0.1:8000")?
.run()
.await
}
