//! Custom Validations for API Fields

extern crate base64;
extern crate sqlx;
extern crate validator;

use base64::Engine;
use sqlx::types::time::OffsetDateTime;
use time::Duration;
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

/// # Schedule Struct
///
/// Parses and Validates the Schedule field in the API.
#[derive(Debug)]
pub struct Schedule {
    /// Occurring (Supported: @every and @once)
    pub routine: String,

    /// Time
    pub time: i64,

    /// Unit of Time (Supported: seconds, minutes, and hours)
    pub timeframe: TimeFrame,
}

#[derive(Debug)]
pub enum TimeFrame {
    Sec,
    Min,
    Hr,
    Day,
}

impl Schedule {
    /// # Parse
    ///
    /// Parses routine, time, and units from a string.
    ///
    /// ## Arguments
    ///
    /// * `input` - A reference to the string input that needs to be parsed.
    ///
    /// ## Returns
    ///
    /// Returns a `Result<Self, ValidationError>` where:
    ///
    /// - `Ok(Self)` contains an instance of the struct if the parsing is successful.
    /// - `Err(ValidationError)` indicates that the input cannot be parsed or is invalid,
    ///   and the specific validation error is provided as an associated value.
    pub fn parse(input: &str) -> Result<Self, ValidationError> {
        let mut parts = input.split_whitespace();

        if let Some(field) = parts.next() {
            if field.starts_with('@') {
                // Parse Routine
                let time_str = parts
                    .next()
                    .ok_or_else(|| ValidationError::new("Missing 'routine' parameter."))?;

                // Parse Time
                let time = time_str.parse::<i64>().map_err(|_| {
                    ValidationError::new(
                        "Invalid 'time' parameter. Ensure that it's a valid integer.",
                    )
                })?;

                // Parse TimeFrame
                let unit = parts
                    .next()
                    .map(|unit| unit.to_string())
                    .ok_or_else(|| ValidationError::new("Missing 'unit' parameter."))?;

                let timeframe = match &unit[..] {
                    "sec" => TimeFrame::Sec,
                    "min" => TimeFrame::Min,
                    "hr" => TimeFrame::Hr,
                    "day" => TimeFrame::Day,
                    _ => {
                        return Err(ValidationError::new(
                            "Invalid 'timeframe' parameter. Valid time frames: sec/min/hr.",
                        ))
                    }
                };

                return Ok(Self {
                    routine: field.to_string(),
                    time,
                    timeframe,
                });
            }
        }

        Err(ValidationError::new("Invalid Syntax for 'schedule' field."))
    }

    /// Calculates Next Run
    pub fn next_run(&self) -> OffsetDateTime {
        let current_time = OffsetDateTime::now_utc();

        match self.timeframe {
            TimeFrame::Sec => current_time + Duration::seconds(self.time),
            TimeFrame::Min => current_time + Duration::minutes(self.time),
            TimeFrame::Hr => current_time + Duration::hours(self.time),
            TimeFrame::Day => {
                let day = self.time * 24;
                current_time + Duration::hours(day)
            }
        }
    }

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
