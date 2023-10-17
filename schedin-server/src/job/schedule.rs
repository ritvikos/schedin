//! Schedule

extern crate std;
extern crate time;
extern crate validator;

use std::str::SplitWhitespace;
use time::{
    macros::{format_description, offset},
    Duration, OffsetDateTime, PrimitiveDateTime,
};
use validator::ValidationError;

/// # Schedule Struct
///
/// Parses and Validates the Schedule field in the API.
#[derive(Debug)]
pub struct Schedule {
    /// Occurrence
    pub routine: Routine,

    /// Time
    pub time: Time,

    /// TimeFrame
    pub timeframe: TimeFrame,
}

#[derive(Debug)]
pub enum Routine {
    Once,
    Every,
    Daily,
    Invalid,
}

#[derive(Debug)]
pub enum Time {
    String(String),
    Integer(i64),
}

#[derive(Debug)]
pub enum TimeFrame {
    Sec,
    Min,
    Hr,
    Day,
    DateTime,
}

impl Schedule {
    // Create a new instance
    pub fn new() -> Self {
        Self {
            routine: Routine::Invalid,
            time: Time::Integer(0),
            timeframe: TimeFrame::Sec,
        }
    }

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
    pub fn parse(&self, input: &str) -> Result<Self, ValidationError> {
        let mut parts = input.split_whitespace();

        match self.parse_routine(&mut parts) {
            Some(Routine::Once) => {
                // check if 'date' is present
                let date = parts
                    .next()
                    .ok_or_else(|| ValidationError::new("Missing 'date' field"))?;

                // check if 'time' is present
                let time = parts
                    .next()
                    .ok_or_else(|| ValidationError::new("Missing 'time' field"))?;

                // create valid datetime format
                let time_str = format!("{} {}", date, time);
                let current_time = OffsetDateTime::now_utc();

                parse_datetime(time_str.trim())?;

                // check if date-time is already elapsed
                if to_datetime(&time_str)
                    .assume_offset(offset!(UTC))
                    .lt(&current_time)
                {
                    println!("current datetime: {}", current_time);
                    return Err(ValidationError::new(
                        "Invalid DateTime: It has already elapsed.",
                    ));
                }

                Ok(Self {
                    routine: Routine::Once,
                    time: Time::String(time_str),
                    timeframe: TimeFrame::DateTime,
                })
            }
            Some(Routine::Every) => {
                // check if 'time' is present
                let time_str = parts
                    .next()
                    .ok_or_else(|| ValidationError::new("Missing 'time'."))?
                    .to_string();

                // check if 'timeframe' is present
                let timeframe_str = parts.next().map(|unit| unit.to_string()).ok_or_else(|| {
                    ValidationError::new("Invalid 'timeframe'. Valid time frames: sec/min/hr.")
                })?;

                // parse 'time' as integer
                let time_int = parse_time(&time_str).ok_or_else(|| {
                    ValidationError::new("Invalid 'time'. It must be an integer.")
                })?;

                // check if 'time' is positive.
                if time_int.is_negative() {
                    return Err(ValidationError::new(
                        "Invalid 'time'. It must be an integer.",
                    ));
                }

                // parse 'timeframe'
                let timeframe = match &timeframe_str[..] {
                    "sec" => TimeFrame::Sec,
                    "min" => TimeFrame::Min,
                    "hr" => TimeFrame::Hr,
                    "day" => TimeFrame::Day,
                    _ => {
                        return Err(ValidationError::new(
                            "Invalid 'timeframe'. Valid time frames: sec/min/hr.",
                        ))
                    }
                };

                println!("time {:?}", time_int);
                println!("timeframe: {:?}", timeframe);

                Ok(Self {
                    routine: Routine::Every,
                    time: Time::Integer(time_int),
                    timeframe,
                })
            }
            _ => Err(ValidationError::new("Invalid 'routine'.")),
        }
    }

    pub fn parse_routine(&self, parts: &mut SplitWhitespace) -> Option<Routine> {
        // check if 'routine' is present
        let routine = if let Some(routine_str) = parts.next() {
            // check if 'routine' starts with '@' symbol
            if routine_str.starts_with('@') {
                // parse 'routine'
                match routine_str {
                    "@once" => Routine::Once,
                    "@every" => Routine::Every,
                    "@daily" => Routine::Daily,
                    _ => Routine::Invalid,
                }
            } else {
                return None;
            }
        } else {
            return None;
        };

        Some(routine)
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

        match &self.time {
            Time::Integer(int) => match self.timeframe {
                TimeFrame::Sec => current_time + Duration::seconds(*int),
                TimeFrame::Min => current_time + Duration::minutes(*int),
                TimeFrame::Hr => current_time + Duration::hours(*int),
                TimeFrame::Day => current_time + Duration::hours(*int * 24),
                _ => panic!(),
            },
            Time::String(str) => to_datetime(str).assume_offset(offset!(UTC)),
        }
    }
}

fn to_datetime(input: &str) -> PrimitiveDateTime {
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    PrimitiveDateTime::parse(input, &format).unwrap()
}

fn parse_time(input: &str) -> Option<i64> {
    if let Ok(int) = input.parse::<i64>() {
        return Some(int);
    }
    None
}

fn parse_datetime(input: &str) -> Result<(), ValidationError> {
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    if let Err(err) = PrimitiveDateTime::parse(input, &format) {
        eprintln!("{}", err);
        return Err(ValidationError::new("Invalid DateTime Format!"));
    };
    Ok(())
}

// fn parse_datetime(input: &str) -> Result<PrimitiveDateTime, ValidationError> {
//     let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
//     println!("{}", input);

//     match PrimitiveDateTime::parse(input, &format) {
//         Ok(datetime) => {
//             println!("datetime: {}", datetime);
//             Ok(datetime)
//         }
//         Err(err) => {
//             eprintln!("{}", err);
//             Err(ValidationError::new("Invalid datetime field"))
//         }
//     }
// }
