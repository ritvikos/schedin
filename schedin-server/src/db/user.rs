//! IAM-related Crud Ops

extern crate sqlx;

use crate::{
    error::CrudError,
    iam::schema::{self},
};
use sqlx::{query, query_as, PgPool, Pool, Postgres, Transaction};

pub struct User {
    pub pool: PgPool,
    pub user: schema::User,
}

impl User {
    /// Default User
    pub fn new(pool: Pool<Postgres>) -> Self {
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

    /// Initialize Database Transaction
    async fn tx(&self) -> Result<Transaction<'static, Postgres>, CrudError> {
        match self.pool.begin().await {
            Ok(tx) => Ok(tx),
            Err(err) => {
                eprintln!("{}", err);
                Err(CrudError::Transaction)
            }
        }
    }

    /// Insert New User
    pub async fn insert(&self) -> Result<(), CrudError> {
        let user_id = self.user.gen_uuid();
        let password = self.user.hash();

        let tx = self.tx().await?;

        query!(
            "INSERT INTO users (user_id, username, passcode, email) VALUES ($1, $2, $3, $4)",
            user_id,
            self.user.username,
            password,
            self.user.email,
        )
        .execute(&self.pool)
        .await
        .unwrap();

        if let Err(err) = tx.commit().await {
            eprintln!("Failed to commit transaction: {}", err);
            return Err(CrudError::Transaction);
        }

        Ok(())
    }

    /// Get User Credentials
    pub async fn credentials(self) -> Result<schema::SigninRow, CrudError> {
        let password = self.user.hash();

        let query = query_as!(
            schema::SigninRow,
            "SELECT user_id, username, passcode FROM users WHERE username=$1 AND passcode=$2",
            self.user.username.unwrap(),
            password
        )
        .fetch_one(&self.pool)
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
