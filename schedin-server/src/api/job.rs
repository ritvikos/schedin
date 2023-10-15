//! Job-Related API Endpoints

extern crate actix_web;
extern crate sqlx;
extern crate std;
extern crate validator;

use crate::{db::DB, iam::schema::AuthorizedUser, job::schema::Job};
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use sqlx::PgPool;
use std::collections::HashMap;
use validator::Validate;

/// # Insert New Job
/// This function inserts a new job into the database based
/// on the provided JSON payload.
///
///
/// ## Job Types
/// - `Binary`: Schedule a binary.
/// - `Code`: Schedule Function.
/// - `Task`: Schedule a Task by name.
///
/// ## Parameters
///
/// - `payload`: A JSON payload containing the information for
/// the job to be inserted.
///
/// ## Errors
///
/// This function may return an HTTP response with an error status code and a corresponding
/// error message if there are issues with the job insertion process.
/// - Invalid payload.
/// - Database is down.
/// - Insufficient permissions.
/// - Internal server errors, etc..
///
/// ## Examples
/// ### Task Job
/// ```json
/// {
///     "name": "job-X",
///     "description": "description_X",
///     "schedule": "@every 10 sec",
///     "task": {
///         "name": "Task_name_1"
///     }
/// }
/// ````
///
/// ### Code Job
/// ```json
/// {
///     "name": "job-Y",
///     "description": "description_Y",
///     "schedule": "@every 10 min",
///     "code": {
///         "src": "base64-encoded-function",
///         "lang": "python",
///         "cmd": "python file.py"
///     }
/// }
/// ````
///
/// ### Binary Job
/// ```json
/// {
///     "name": "job-Z",
///     "description": "description_Z",
///     "schedule": "@every 10 hrs",
///     "bin": {
///         "path": "https://s3-bucket.com/1"
///     }
/// }
/// ````
pub async fn insert_job(
    account: AuthorizedUser,
    payload: Json<Job>,
    db: Data<PgPool>,
) -> impl Responder {
    if let Err(err) = payload.validate() {
        return HttpResponse::BadRequest().json(err);
    }

    if let Err(err) = DB::new(db.into_inner())
        .job(payload.0)
        .insert(&account.id)
        .await
    {
        return err.json();
    }

    let mut map = HashMap::with_capacity(1);
    map.insert("status", "ok");
    HttpResponse::Ok().json(map)
}

/// # Delete Job
/// This function deletes an existing job from the database based
/// on the provided JSON payload.
///
/// ## Parameters
///
/// - `payload`: A JSON payload containing the information for
/// the job to be deleted.
///
/// ## Errors
///
/// This function may return an HTTP response with an error status code and a corresponding
/// error message if there are issues with the job deletion.
/// - Invalid payload.
/// - Database is down.
/// - Insufficient permissions.
/// - Internal server errors, etc..
///
/// ## Example
/// ```json
/// {
///     "name": "job-X"
/// }
/// ```
pub async fn delete_job(
    account: AuthorizedUser,
    payload: Json<Job>,
    db: Data<PgPool>,
) -> impl Responder {
    if let Err(err) = DB::new(db.into_inner())
        .job(payload.0)
        .delete(account.id)
        .await
    {
        return err.json();
    }

    let mut map = HashMap::with_capacity(1);
    map.insert("status", "ok");
    HttpResponse::Ok().json(map)
}
