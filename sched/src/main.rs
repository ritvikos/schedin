//! Scheduling Service

extern crate actix_web;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use certs::load_rustls_config;

mod certs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tls_config = load_rustls_config();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1").route("/test", web::get().to(test_endpoint)))
            .wrap(middleware::NormalizePath::default())
    })
    .bind_rustls("localhost:8080", tls_config)?
    .run()
    .await
}

async fn test_endpoint() -> impl Responder {
    HttpResponse::Ok().json("ok!")
}
