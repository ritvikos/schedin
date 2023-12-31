//! Orchestrator

extern crate schedin_common;
extern crate sqlx;
extern crate std;
extern crate tokio;

mod db;
mod job;

use db::DB;
use schedin_common::db::create_pool;
use sqlx::Postgres;
use std::{env, thread, time::Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    loop {
        let pool = create_pool::<Postgres>(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let db = DB::new(pool);

        if let Ok(jobs) = db.read(Duration::from_secs(600)).await {
            println!("{:?}", jobs);
        }

        thread::sleep(Duration::from_secs(60))
    }
}
