use std::fmt::Display;

#[derive(Debug)]
pub enum ScheduleParseError {
    /// Must be inserted in the actual order, where `from` must be a lower value than `to`.
    BadRange{from: String, to: String},
    InvalidToken(String),
    InvalidValue {field: &'static str, value: u8},
    TooShort
}

impl Display for ScheduleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRange {from, to} => write!(f, "range is invalid, {to} cannot come before {from}"),
            Self::InvalidToken(token) => write!(f, "{token} is not a valid schedule value"),
            Self::InvalidValue { field, value } => write!(f, "\"{value}\" is not a valid {field}"),
            Self::TooShort => f.write_str("Schedule is not long enough, must be at least \"* * *\"")
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