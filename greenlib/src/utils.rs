use std::fmt::Write;
use std::str::FromStr;

use crate::errors::Error;

/// Helper function for providing richer feedback when attempting to parse an 
/// integer from a str. Provides more user-readable error messaging than 
/// ParseIntError provides.
/// 
/// Could fail if fmt fails.
pub(crate) fn parse_int<S: AsRef<str>, T: FromStr>(s: S) -> Result<T, Error> {
    let s = s.as_ref();
    let Ok(val) = s.parse() else {
        // little hidden helper function, might as well avoid additional heap 
        // allocations without cluttering the rest of the code, yay efficiency!
        let p1 = "could not parse ";
        let p2 = " as an integer";

        let mut msg = String::with_capacity(p1.len() + s.len() + p2.len()); // single byte chars
        write!(msg, "{}{}{}", p1, s, p2)?;
        return Err(Error::ParseError(msg));
    };

    Ok(val)
}