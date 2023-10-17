//! Custom Validations for API Fields

extern crate base64;
extern crate validator;

use crate::job::schedule::Schedule;
use base64::Engine;
use validator::ValidationError;

/// # Validate Schedule
///
/// Validates a schedule represented as a string to ensure it adheres to a specific format.
///
/// ## Arguments
///
/// `input` - Schedule string that needs validation.
///
/// ## Returns
///
/// Returns a `Result<(), ValidationError>` where:
///
/// - `Ok(())` indicates that the schedule is valid.
///
/// - `Err(ValidationError)` indicates that the schedule is invalid, and the corresponding
///   validation error is provided as an associated value.
///
pub fn validate_schedule(input: &str) -> Result<(), ValidationError> {
    Schedule::new(input).parse()?;
    Ok(())
}

/// # Validate Source Format
/// Ensure the source code is valid base64-encoded string
pub fn validate_source_format(input: &str) -> Result<(), ValidationError> {
    if base64::engine::general_purpose::STANDARD
        .decode(input)
        .is_err()
    {
        return Err(ValidationError::new("Invalid Base64-encoded source code"));
    }

    Ok(())
}
