//! Custom Error Types

extern crate std;

use std::collections::HashMap;

/// CRUD Errors
#[derive(Debug)]
pub enum CrudError {
    /// Error during data insertion
    Insertion,

    /// Error creating database pool
    Pooling,

    /// Error creating database transaction
    Transaction,

    /// Validation error for user-provided data
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

    pub fn map(&self) -> HashMap<&str, &str> {
        match self {
            CrudError::Insertion => error_map(CrudError::Insertion),
            CrudError::Pooling => error_map(CrudError::Pooling),
            CrudError::Read => error_map(CrudError::Read),
            CrudError::Transaction => error_map(CrudError::Transaction),
            CrudError::Validation => error_map(CrudError::Validation),
        }
    }
}

/// Common error response map
pub fn error_map<'a>(error: CrudError) -> HashMap<&'a str, &'a str> {
    eprintln!("Internal Error: {:?}", error);

    let mut map = HashMap::with_capacity(2);
    map.insert("status", "error");
    map.insert("reason", error.reason());

    map
}
