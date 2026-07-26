/* A bit of boilerplate in the is_set and next_set functions here, unfortunately. 
While each period (DayOfMonth, DayOfWeek, etc.) could be defined by a single u64 
wrapper type with shared functions (and only a small amount of overhead, for 
actual bytes needed), the need to provide each type with a distinct MAX value 
and means of parsing `FromStr` (e.g. due to only day and month requiring 
skipping parsing `0`) meant a struct for each was the most logical option. */
mod dom;
mod errors;
mod schedule;
pub(crate) mod utils;

pub use dom::DayOfMonth;
pub use errors::Error;
pub use schedule::{ExceptSchedule, Schedule};