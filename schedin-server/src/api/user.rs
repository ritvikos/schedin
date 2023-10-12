//! IAM-Related API Endpoints

extern crate actix_web;
extern crate futures;
extern crate jsonwebtoken;
extern crate sqlx;
extern crate std;

use crate::{
    db,
    iam::{
        schema::{AuthorizedUser, SigninResponse, User},
        token::Claims,
    },
};
use actix_web::{
    dev::Payload,
    error::ErrorBadRequest,
    http::header,
    web::{Data, Json},
    FromRequest, HttpRequest, HttpResponse, Responder,
};
use futures::Future;
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::PgPool;
use std::{collections::HashMap, env, pin::Pin};

/// # Sign-Up
/// ## Insert New User
/// This function inserts a new user into the database based
/// on the provided JSON payload.
///
/// ## Parameters
///
/// - `payload`: A JSON payload containing the user data to be inserted.
///
/// ## Errors
///
/// This function may return an HTTP response with an error status code and a corresponding
/// error message if there are issues with the job insertion process.
/// - Invalid payload.
/// - Database is down.
/// - Internal server errors, etc...
pub async fn signup(payload: Json<User>, db: Data<PgPool>) -> impl Responder {
    let pool = db.into_inner().as_ref().clone();

    // Insert new user
    let user = db::user::User::new(pool).user(payload.0).insert().await;

    if let Err(err) = user {
        return err.json();
    }

    let mut map = HashMap::with_capacity(1);
    map.insert("status", "ok");
    HttpResponse::Ok().json(map)
}

/// # Sign-In
/// This function retrieves user from database and validates credentials.
///
/// ## Parameters
///
/// - `payload`: A JSON payload containing the user data on the basis of which the
/// credentials will be validated.
///
/// ## Errors
///
/// This function may return an HTTP response with an error status code and a corresponding
/// error message if there are issues with the job insertion process.
/// - Invalid payload.
/// - Database is down.
/// - Internal server errors, etc...
pub async fn signin(payload: Json<User>, db: Data<PgPool>) -> impl Responder {
    let pool = db.into_inner().as_ref().clone();

    // Try to retrieve user credentials from the database
    let user = db::user::User::new(pool)
        .user(payload.0)
        .credentials()
        .await;

    match user {
        Ok(fields) => {
            let id = fields.user_id;
            let token = fields.gen_token();

            let response = SigninResponse::new()
                .message("Sign-in successful")
                .username(fields.username)
                .id(id)
                .token(token);

            HttpResponse::Ok().json(response)
        }
        Err(err) => err.json(),
    }
}

impl FromRequest for AuthorizedUser {
    type Error = actix_web::error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Try to retrieve `authorization` token from headers
        let token = match req.headers().get(header::AUTHORIZATION) {
            Some(header) => header.to_str(),
            None => {
                let error = actix_web::error::ErrorBadRequest("No Bearer Token found!");
                return Box::pin(async move { Err(error) });
            }
        }
        .unwrap();

        // Try to decode the claims and headers
        let decoded_token = decode::<Claims>(
            &token[7..],
            &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
            &Validation::default(),
        );

        match decoded_token {
            Ok(token) => {
                let authorized_user = Self {
                    id: token.claims.sub,
                };

                Box::pin(async move { Ok(authorized_user) }) as _
            }
            Err(_) => {
                // todo!(): Robust Error Handling

                Box::pin(
                    async move { Err(ErrorBadRequest("Token Expired! Please Sign-In Again.")) },
                )
            }
        }
    }
}
