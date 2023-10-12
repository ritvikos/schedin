//! Custom Error Types
//! Contains both API and Database related errors.

extern crate actix_web;
extern crate std;

use actix_web::HttpResponse;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(unused)]
/// CRUD related Errors
pub enum CrudError {
    /// Error during data insertion.
    Insertion,

    /// Error creating database pool.
    Pooling,

    /// Error creating database transaction.
    Transaction,

    /// Validation error for user-provided data.
    Validation,

    /// Error when reading from database
    Read,
}

impl CrudError {
    pub fn reason(self) -> &'static str {
        match self {
            CrudError::Insertion => "Cannot Insert into Database",
            CrudError::Pooling => "Unable to Pool Database",
            CrudError::Read => "Unable to Read from Database",
            CrudError::Transaction => "Unable to create Transaction",
            CrudError::Validation => "Invalid JSON Parameters",
        }
    }

    pub fn json(&self) -> HttpResponse {
        match self {
            CrudError::Insertion => error_response(CrudError::Insertion),
            CrudError::Pooling => error_response(CrudError::Pooling),
            CrudError::Read => error_response(CrudError::Read),
            CrudError::Transaction => error_response(CrudError::Transaction),
            CrudError::Validation => error_response(CrudError::Validation),
        }
    }
}

/// Common Error Response for the API based on the error type.
pub fn error_response(error: CrudError) -> HttpResponse {
    eprintln!("Internal Error: {:?}", error);

    let mut map = HashMap::with_capacity(2);
    map.insert("status", "error");
    map.insert("reason", error.reason());

    HttpResponse::InternalServerError().json(map)
}
