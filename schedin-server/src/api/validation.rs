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
/// * `input` - Schedule string that needs validation.
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
    let schedule = Schedule::parse(input)?;
    schedule.validate()
}

impl Schedule {
    /// # Validate
    ///
    /// Validates the state of a struct object, ensuring that it meets the required criteria by
    /// invoking corresponding validation methods for routine, time, and unit properties.
    ///
    /// ## Returns
    ///
    /// Returns a `Result<(), ValidationError>` where:
    ///
    /// - `Ok(())` indicates that the struct object is valid.
    /// - `Err(ValidationError)` indicates that the struct object is invalid, and the corresponding
    ///   validation error is provided as an associated value.
    fn validate(&self) -> Result<(), ValidationError> {
        self.validate_routine()?;
        self.validate_time()?;

        Ok(())
    }

    /// # Validate Routine
    ///
    /// Validates the state of a routine object, ensuring:
    /// * It's either @every or @once
    ///
    /// ## Returns
    ///
    /// Returns a `Result<(), ValidationError>` where:
    ///
    /// - `Ok(())` indicates that the routine is valid.
    /// - `Err(ValidationError)` indicates that the routine is invalid, and the corresponding
    ///   validation error is provided as an associated value.
    ///
    fn validate_routine(&self) -> Result<(), ValidationError> {
        match &self.routine[..] {
            "@every" | "@once" => Ok(()),
            _ => Err(ValidationError::new("Invalid routine parameter.")),
        }
    }

    /// # Validate Time
    ///
    /// Validates the time field within a struct, ensuring:
    /// * It's valid integer greater than 0.
    ///
    /// ## Returns
    ///
    /// Returns a `Result<(), ValidationError>` where:
    ///
    /// - `Ok(())` indicates that the time value is valid.
    /// - `Err(ValidationError)` indicates that the time value is invalid, and the corresponding
    ///   validation error is provided as an associated value.
    fn validate_time(&self) -> Result<(), ValidationError> {
        if !self.time.gt(&0) {
            return Err(ValidationError::new("Invalid time parameter."));
        }

        Ok(())
    }
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
