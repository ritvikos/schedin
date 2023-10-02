//! Scheduling Service

extern crate actix_web;
extern crate sqlx;
extern crate std;

use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use certs::load_rustls_config;
use sqlx::{migrate::Migrator, PgPool};
use std::env;

mod certs;

static MIGRATOR: Migrator = sqlx::migrate!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tls_config = load_rustls_config();

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    MIGRATOR.run(&pool).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api/v1").route("/test", web::get().to(test_endpoint)))
            .wrap(middleware::NormalizePath::default())
            .app_data(Data::new(pool.clone()))
    })
    .bind_rustls("localhost:8080", tls_config)?
    .run()
    .await
}

async fn test_endpoint() -> impl Responder {
    HttpResponse::Ok().json("ok!")
}
