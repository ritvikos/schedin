//! User Management
//! Unified Schema for API and Database

extern crate hex;
extern crate serde;
extern crate sha2;
extern crate uuid;

use hex::encode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Default, Deserialize)]
pub struct User {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl User {
    /// # Generate UUID
    /// Generate UUID V4
    ///
    /// ## Returns
    /// Uuid
    pub fn gen_uuid(&self) -> Uuid {
        Uuid::new_v4()
    }

    /// # Hash Password
    /// Hashes password with SHA256 algo
    ///
    /// ## Returns
    /// Hex Encoded String
    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.password.as_ref().unwrap().as_bytes());
        encode(hasher.finalize())
    }
}

/// # Sign-in Response
/// Success response after sign-in
#[derive(Debug, Serialize, Deserialize)]
pub struct SigninResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub username: String,
    pub token: String,
}

impl SigninResponse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message<S>(mut self, message: S) -> Self
    where
        S: ToString,
    {
        self.message = message.to_string();
        self
    }

    pub fn id<S>(mut self, id: S) -> Self
    where
        S: ToString,
    {
        self.id = id.to_string();
        self
    }

    pub fn username(mut self, username: String) -> Self {
        self.username = username;
        self
    }

    pub fn token(mut self, token: String) -> Self {
        self.token = token;
        self
    }
}

impl Default for SigninResponse {
    fn default() -> Self {
        Self {
            success: true,
            message: String::with_capacity(20),
            id: String::with_capacity(36),
            username: String::with_capacity(10),
            token: String::with_capacity(150),
        }
    }
}

/// Authorized User
///
/// Represents an authorized user after decoding and validating a JWT token.
pub struct AuthorizedUser {
    pub id: String,
}

/// Represents user sign-in information retrieved from the database.
///
/// This data is typically retrieved from a database when a user
/// attempts to sign in and is used to validate their identity.
#[derive(sqlx::FromRow)]
pub struct SigninRow {
    pub user_id: Uuid,
    pub username: String,
    pub passcode: String,
}
