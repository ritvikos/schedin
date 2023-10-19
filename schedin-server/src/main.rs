//! Scheduling Service

extern crate actix_web;
extern crate schedin_common;
extern crate sqlx;
extern crate std;

use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use api::{
    job::{delete_job, insert_job},
    user::{signin, signup},
};
use certs::load_rustls_config;
use iam::schema::AuthorizedUser;
use schedin_common::db::create_pool;
use sqlx::{migrate::Migrator, Postgres};
use std::env;

mod api;
mod certs;
mod db;
mod iam;
mod job;

static MIGRATOR: Migrator = sqlx::migrate!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tls_config = load_rustls_config();

    let pool = create_pool::<Postgres>(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    MIGRATOR.run(&pool).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .route("/test", web::get().to(test_endpoint))
                    .service(
                        web::scope("/user")
                            .route("/signup", web::post().to(signup))
                            .route("/signin", web::post().to(signin)),
                    )
                    .service(
                        web::scope("/job")
                            .route("/new", web::post().to(insert_job))
                            .route("/delete", web::post().to(delete_job)),
                    ),
            )
            .wrap(middleware::NormalizePath::default())
            .app_data(Data::new(pool.clone()))
    })
    .bind_rustls("localhost:8080", tls_config)?
    .run()
    .await
}

async fn test_endpoint(user: AuthorizedUser) -> impl Responder {
    println!("user.id = {}", user.id);
    HttpResponse::Ok().json("ok!")
}
