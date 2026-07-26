use std::ops::{Deref, DerefMut};

use crate::{Error, utils};

const DOM_MAX: u32 = 31; // u32 for quick comp on return on trailing_zeros().

#[derive(Debug)]
pub struct DayOfMonth {
    bits: u32
}

impl DayOfMonth {
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

        if next >= DOM_MAX { return None; }

        Some(next)
    }

    /// Update the bitmask for this `DayOfMonth` by attempting to parse from and 
    /// until `str` values. This function expects callers to have already split 
    /// the range into two parts by "-".
    ///  
    /// Errors if the values can't be parsed as integers, if `from` is greater 
    /// than `until` or if `until` is greater than [`DOM_MAX`].
    pub fn set_range(
        &mut self, from: &str, until: &str
    ) -> Result<(), Error> {
        let from: u32 = utils::parse_int(from)?;
        let until: u32 = utils::parse_int(until)?;

        if from > until {
            let msg = format!("Could not parse {}-{} as a valid range", from, until);
            return Err(Error::ParseError(msg.into()))
        }

        if until >= DOM_MAX {
            let msg = format!(
                "Could not parse {}-{} as a valid range. Day can be max {}",
                from, until, DOM_MAX
            );
            return Err(Error::ParseError(msg.into()))
        }

        for dom in from..=until {
            self.bits |= 1u32 << dom;
        }

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