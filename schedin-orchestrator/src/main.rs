//! Orchestrator

extern crate sqlx;
extern crate std;
extern crate tokio;

mod db;
mod error;
mod job;

use db::{create_pool, DB};
use sqlx::Postgres;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let pool = create_pool::<Postgres>(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let db = DB::new(pool);

    if let Ok(jobs) = db.read_all().await {
        println!("{:?}", jobs);
    }
}
