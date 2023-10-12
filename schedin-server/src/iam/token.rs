//! JWT Token

extern crate jsonwebtoken;
extern crate serde;
extern crate std;
extern crate time;

use super::schema::SigninRow;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};
use time::OffsetDateTime;

/// JWT Token Claims
#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    /// Expiration Time (UNIX Timestamp)
    pub exp: usize,

    /// User ID
    pub sub: String,
}

impl SigninRow {
    /// Generate JWT Token
    pub fn gen_token(&self) -> String {
        let current_time = OffsetDateTime::now_utc();

        // Default Expiration Time: 1 hour from the current time
        let expiration_time = current_time + Duration::from_secs(3600);
        let expiration_timestamp = expiration_time.unix_timestamp() as usize;

        let claims = Claims {
            sub: self.user_id.to_string(),
            exp: expiration_timestamp,
        };

        // Encode the headers and claims
        match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&env::var("JWT_SECRET").unwrap().into_bytes()),
        ) {
            Ok(t) => t,
            Err(_) => panic!(),
        }
    }
}
