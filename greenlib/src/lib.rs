pub mod errors;
mod schedule;

pub use errors::LibError;
pub use schedule::{today::Date, Schedule};

pub(crate) type LibResult<T> = Result<T, errors::LibError>;