//! Schedule

extern crate time;
extern crate validator;

use time::{Duration, OffsetDateTime};
use validator::ValidationError;

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
    /// Parses routine, time, and timeframe from a string.
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
                let unit = parts.next().map(|unit| unit.to_string()).ok_or_else(|| {
                    ValidationError::new(
                        "Invalid 'timeframe' parameter. Valid time frames: sec/min/hr.",
                    )
                })?;

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

    /// # Next Run
    /// Calculates the next timestamp based on the provided `TimeFrame` and time duration (in UTC).
    ///
    /// This function takes the current time in UTC and adds a specified time duration based on
    /// the selected `TimeFrame`. The result is returned as an `OffsetDateTime`.
    ///
    /// # Returns
    ///
    /// - An `OffsetDateTime` which represents the calculated next timestamp.
    pub fn next_run(&self) -> OffsetDateTime {
        let current_time = OffsetDateTime::now_utc();

        match self.timeframe {
            TimeFrame::Sec => current_time + Duration::seconds(self.time),
            TimeFrame::Min => current_time + Duration::minutes(self.time),
            TimeFrame::Hr => current_time + Duration::hours(self.time),
            TimeFrame::Day => current_time + Duration::hours(self.time * 24),
        }
    }
}
