//! IAM-related Crud Ops

extern crate schedin_common;
extern crate sqlx;
extern crate std;

use crate::iam::schema::{self};
use schedin_common::{error::CrudError, tx::Tx};
use sqlx::{query, query_as, PgPool, Pool, Postgres};
use std::sync::Arc;

pub struct User {
    pub pool: Arc<PgPool>,
    pub user: schema::User,
}

impl User {
    /// New User
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            pool,
            user: schema::User::default(),
        }
    }

    /// Sets and returns modified user
    pub fn user(mut self, user: schema::User) -> Self {
        self.user = user;
        self
    }

    /// Insert New User
    pub async fn insert(&self) -> Result<(), CrudError> {
        let user_id = self.user.gen_uuid();
        let password = self.user.hash();

        let tx_manager = Tx::new(self.pool.clone());
        let tx = tx_manager.init().await?;

        query!(
            r#"
            INSERT INTO users (user_id, username, passcode, email) 
            VALUES ($1, $2, $3, $4)
            "#,
            user_id,
            self.user.username,
            password,
            self.user.email,
        )
        .execute(&*self.pool)
        .await
        .unwrap();

        tx_manager.commit(tx).await?;

        Ok(())
    }

    /// Get User Credentials
    pub async fn credentials(self) -> Result<schema::SigninRow, CrudError> {
        let password = self.user.hash();

        let query = query_as!(
            schema::SigninRow,
            r#"
            SELECT user_id, username, passcode FROM users 
            WHERE username=$1 AND passcode=$2
            "#,
            self.user.username.unwrap(),
            password
        )
        .fetch_one(&*self.pool)
        .await;

        match query {
            Ok(row) => Ok(row),
            Err(err) => {
                eprintln!("{}", err);
                Err(CrudError::Read)
            }
        }
    }
}
