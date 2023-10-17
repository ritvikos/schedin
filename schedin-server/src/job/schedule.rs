//! Schedule

extern crate std;
extern crate time;
extern crate validator;

use core::str::SplitWhitespace;
use time::{
    macros::{format_description, offset},
    Duration, OffsetDateTime, PrimitiveDateTime,
};
use validator::ValidationError;

/// # ParsedSchedule Struct
///
/// Parses and Validates the Schedule field in the API.
#[derive(Debug)]
pub struct ScheduleParser {
    /// Occurrence
    pub routine: Routine,

    // Timestamp
    pub timestamp: Timestamp,
}

#[derive(Debug)]
pub enum Routine {
    Once,
    Every,
    Daily,
    Invalid,
}

#[derive(Debug)]
pub struct Timestamp {
    pub time: Time,
    pub timeframe: Timeframe,
}

#[derive(Debug)]
pub enum Time {
    Timestamp(OffsetDateTime),
    Integer(i64),
}

#[derive(Debug)]
pub enum Timeframe {
    Sec,
    Min,
    Hr,
    Day,
    DateTime,
}

pub struct Schedule<'a>(SplitWhitespace<'a>);

impl Schedule<'_> {
    pub fn new(input: &str) -> Schedule<'_> {
        Schedule(input.split_whitespace())
    }

    /// # Parse
    /// Parses routine, time, and timeframe from a string.
    ///
    /// ## Returns
    ///
    /// Returns a `Result<Self, ValidationError>` where:
    ///
    /// - `Ok(Self)` contains an instance of the struct if the parsing is successful.
    /// - `Err(ValidationError)` indicates that the input cannot be parsed or is invalid,
    ///   and the specific validation error is provided as an associated value.
    pub fn parse(mut self) -> Result<ScheduleParser, ValidationError> {
        match self.routine() {
            Ok(routine) => {
                let result = match routine {
                    Routine::Once => TimestampParser::new(self.0).datetime(),
                    Routine::Every => TimestampParser::new(self.0).interval(),
                    _ => {
                        return Err(ValidationError::new("Invalid 'routine'"));
                    }
                };

                match result {
                    Ok(timestamp) => Ok(ScheduleParser { routine, timestamp }),
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }

    /// Parse 'routine' parameter
    pub fn routine(&mut self) -> Result<Routine, ValidationError> {
        // check if present
        let routine = if let Some(routine_str) = self.0.next() {
            if routine_str.starts_with('@') {
                match routine_str {
                    "@once" => Routine::Once,
                    "@every" => Routine::Every,
                    "@daily" => Routine::Daily,
                    _ => Routine::Invalid,
                }
            } else {
                return Err(ValidationError::new(
                    "Invalid syntax for 'routine' parameter",
                ));
            }
        } else {
            return Err(ValidationError::new("Missing 'routine' parameter"));
        };

        Ok(routine)
    }
}

pub struct TimestampParser<'a>(SplitWhitespace<'a>);

impl Default for Timestamp {
    fn default() -> Self {
        Self {
            time: Time::Integer(0),
            timeframe: Timeframe::Sec,
        }
    }
}

impl<'a> TimestampParser<'a> {
    pub fn new(inner: SplitWhitespace<'a>) -> Self {
        TimestampParser(inner)
    }

    pub fn datetime(&mut self) -> Result<Timestamp, ValidationError> {
        // check if 'date' is present
        let date = self
            .0
            .next()
            .ok_or_else(|| ValidationError::new("Missing 'date' field"))?;

        // check if 'time' is present
        let time = self
            .0
            .next()
            .ok_or_else(|| ValidationError::new("Missing 'time' field"))?;

        // create valid timestamp
        let timestamp_str = format!("{} {}", date, time);
        let current_timestamp = OffsetDateTime::now_utc();

        // check if valid timestamp
        let timestamp = to_timestamp(timestamp_str.trim())?;

        // check if date-time is already elapsed
        if timestamp.lt(&current_timestamp) {
            println!("current datetime: {}", current_timestamp);
            return Err(ValidationError::new(
                "Invalid DateTime: It has already elapsed.",
            ));
        }
        Ok(Timestamp {
            time: Time::Timestamp(timestamp),
            timeframe: Timeframe::DateTime,
        })
    }

    pub fn interval(&mut self) -> Result<Timestamp, ValidationError> {
        // check if 'time' is present
        let time_str = self
            .0
            .next()
            .ok_or_else(|| ValidationError::new("Missing 'time'."))?
            .to_string();

        // check if 'timeframe' is present
        let timeframe_str = self.0.next().map(|unit| unit.to_string()).ok_or_else(|| {
            ValidationError::new("Invalid 'timeframe'. Valid time frames: sec/min/hr.")
        })?;

        // parse 'time' as integer
        let mut duration = parse_time(&time_str)
            .ok_or_else(|| ValidationError::new("Invalid 'time'. It must be an integer."))?;

        // check if 'time' is positive.
        if duration.is_negative() {
            return Err(ValidationError::new(
                "Invalid 'time'. It must be an integer.",
            ));
        }

        // parse 'timeframe'
        let timeframe = match &timeframe_str[..] {
            "sec" => Timeframe::Sec,
            "min" => {
                duration *= 60;
                Timeframe::Min
            }
            "hr" => {
                duration *= 60 * 60;
                Timeframe::Hr
            }
            "day" => {
                duration *= 60 * 60 * 24;
                Timeframe::Day
            }
            _ => {
                return Err(ValidationError::new(
                    "Invalid 'timeframe'. Valid time frames: sec/min/hr.",
                ))
            }
        };

        Ok(Timestamp {
            time: Time::Integer(duration),
            timeframe,
        })
    }
}

impl ScheduleParser {
    /// # Next Run
    /// Calculates the next timestamp based on the provided `Timeframe` and time duration (in UTC).
    ///
    /// This function takes the current time in UTC and adds a specified time duration based on
    /// the selected `Timeframe`. The result is returned as an `OffsetDateTime`.
    ///
    /// # Returns
    ///
    /// - An `OffsetDateTime` which represents the calculated next timestamp.
    pub fn next_run(&self) -> OffsetDateTime {
        let current_time = OffsetDateTime::now_utc();

        match &self.timestamp.time {
            Time::Integer(int) => match self.timestamp.timeframe {
                Timeframe::Sec | Timeframe::Min | Timeframe::Hr | Timeframe::Day => {
                    current_time + Duration::seconds(*int)
                }
                _ => panic!(),
            },
            Time::Timestamp(timestamp) => *timestamp,
        }
    }
}

impl Default for ScheduleParser {
    fn default() -> Self {
        Self {
            routine: Routine::Invalid,
            timestamp: Timestamp {
                time: Time::Integer(0),
                timeframe: Timeframe::Sec,
            },
        }
    }
}

fn to_timestamp(input: &str) -> Result<OffsetDateTime, ValidationError> {
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    match PrimitiveDateTime::parse(input, &format) {
        Ok(datetime) => Ok(datetime.assume_offset(offset!(UTC))),
        Err(_) => Err(ValidationError::new("Invalid DateTime format")),
    }
}

fn parse_time(input: &str) -> Option<i64> {
    if let Ok(int) = input.parse::<i64>() {
        return Some(int);
    }
    None
}
