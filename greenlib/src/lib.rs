pub mod errors;
mod schedule;

pub use errors::LibError;

pub(crate) type LibResult<T> = Result<T, errors::LibError>;