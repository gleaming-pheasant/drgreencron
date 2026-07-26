use std::fmt::Display;

/// Errors from this crate are designed to be readable to end users for output 
/// to the terminal (or other output), not developers.
/// 
/// Each variant simply wraps a message which varies by scenario (e.g. parsing a 
/// `Schedule` generally, vs parsing a `DayOfMonth` range), and is provided as a 
/// variant predominantly for debugging purposes.
#[derive(Debug)]
pub enum Error {
    FmtError(std::fmt::Error),
    ParseError(String)
}

impl From<std::fmt::Error> for Error {
    fn from(value: std::fmt::Error) -> Self {
        Self::FmtError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // something has gone wrong if we get to here!
            Self::FmtError(e) => write!(f,"{e}"),
            Self::ParseError(msg) => f.write_str(&msg)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::FmtError(e) => Some(e),
            Error::ParseError(_) => None
        }
    }
}