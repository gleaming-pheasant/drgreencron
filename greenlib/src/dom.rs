use std::ops::{Deref, DerefMut};

use crate::{Error, utils};

const DOM_MIN: u8 = 1;
const DOM_MAX: u8 = 31;

/// DayOfMonth parses values as their direct u8 representation, so bits at index 
/// 0 should never be set.
/// 
/// E.g. if range 1-5 is given, bits 1-5 will be set, not 0-4.
#[derive(Debug)]
pub struct DayOfMonth {
    bits: u32
}

impl DayOfMonth {
    /// Create a new `DayOfMonth`, with all `bits` set to 0.
    #[inline]
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    /// Returns `true` if the given bit `position` is set.
    #[inline]
    pub fn is_set(&self, position: u32) -> bool {
        self.bits & (1u32 << position) != 0
    }

    /// Returns the next value in this `CronField` which is set after the 
    /// provided position, if there is another bit set (otherwise returns 
    /// `None`).
    pub fn next_set(&self, from_position: u32) -> Option<u32> {
        let shifted = self.bits >> from_position;
        if shifted == 0 { return None; }

        let next = from_position + shifted.trailing_zeros();

        if next >= DOM_MAX as u32 { return None; }

        Some(next)
    }

    /// Update the bitmask for this `DayOfMonth` by attempting to parse from and 
    /// until `str` values. This function expects callers to have already split 
    /// the range into two parts by "-".
    ///  
    /// Errors if the values can't be parsed as integers, if `from` is greater 
    /// than `until` or if `until` is greater than or equal to [`DOM_MAX`].
    pub fn set_range(
        &mut self, from: &str, until: &str
    ) -> Result<(), Error> {
        let range = DOM_MIN..=DOM_MAX;
        
        let from: u8 = utils::parse_int(from)?;
        let until: u8 = utils::parse_int(until)?;

        if !range.contains(&from) || !range.contains(&until) {
            let msg = format!("Day not between {} and {}", DOM_MIN, DOM_MAX);
            return Err(Error::ParseError(msg))
        }

        if from >= until {
            let msg = format!(
                "Could not parse {}-{}, {} must be smaller than {}",
                from, until, from, until
            );
            return Err(Error::ParseError(msg.into()))
        }

        for dom in from..=until {
            self.bits |= 1u32 << dom;
        }

        Ok(())
    }

    pub fn set_digit(&mut self, day: &str) -> Result<(), Error> {
        let range = DOM_MIN..=DOM_MAX;
        
        let day: u8 = utils::parse_int(day)?;

        if !range.contains(&day) {
            let msg = format!("Day \"{}\" not between {} and {}", day, DOM_MIN, DOM_MAX);
            return Err(Error::ParseError(msg))
        }

        self.bits |= 1u32 << day;

        Ok(())
    }
}

impl Deref for DayOfMonth {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.bits
    }
}

impl DerefMut for DayOfMonth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bits
    }
}