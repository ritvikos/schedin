//! Custom Error Types

#![allow(unused)]

#[derive(Debug)]
/// CRUD related Errors
pub enum CrudError {
    /// Error creating database pool
    Pooling,

    /// Error reading from database
    Read,

    /// Error creating database transaction
    Transaction,
}

impl CrudError {
    /// Reason for error
    pub fn reason(self) -> &'static str {
        match self {
            CrudError::Pooling => "Unable to Pool Database",
            CrudError::Read => "Unable to Read from Database",
            CrudError::Transaction => "Unable to create Transaction",
        }
    }
}
