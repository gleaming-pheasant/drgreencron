use std::fmt::Display;

#[derive(Debug)]
pub enum ScheduleParseError {
    /// Must be inserted in the actual order, where `from` must be a lower value than `to`.
    BadRange,
    InvalidDayOfWeek,
    InvalidDigit,
    InvalidValue,
    TooLong,
    TooShort
}

impl Display for ScheduleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRange => f.write_str("a range must begin with the lower value"),
            Self::InvalidDayOfWeek => f.write_str("encountered invalid day of the week name"),
            Self::InvalidDigit => f.write_str("encountered invalid digit in schedule"),
            Self::InvalidValue => f.write_str("encountered invalid value in schedule"),
            Self::TooLong => f.write_str("schedule is too long"),
            Self::TooShort => f.write_str("schedule is not long enough, must be at least \"* * *\"")
        }
    }
}

/// Errors from this crate are designed to be readable to end users for output 
/// to the terminal (or other output), not developers.
/// 
/// Each variant simply wraps a message which varies by scenario (e.g. parsing a 
/// `Schedule` generally, vs parsing a `DayOfMonth` range), and is provided as a 
/// variant predominantly for debugging purposes.
#[derive(Debug)]
pub enum LibError {
    FmtError(std::fmt::Error),
    ScheduleParseError(ScheduleParseError)
}

impl From<ScheduleParseError> for LibError {
    fn from(value: ScheduleParseError) -> Self {
        Self::ScheduleParseError(value)
    }
}

// just here for ease of using `?` propagator. If this error is returned, the 
// Error itself is in trouble!
impl From<std::fmt::Error> for LibError {
    fn from(value: std::fmt::Error) -> Self {
        Self::FmtError(value)
    }
}

impl Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // something has gone wrong if we get to here!
            Self::FmtError(e) => write!(f,"{e}"),
            Self::ScheduleParseError(msg) => msg.fmt(f)
        }
    }
}

impl std::error::Error for LibError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FmtError(e) => Some(e),
            Self::ScheduleParseError(_) => None
        }
    }
}