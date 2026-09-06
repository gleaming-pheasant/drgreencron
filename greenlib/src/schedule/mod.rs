use std::str::FromStr;

use crate::{LibError, LibResult, errors::ScheduleParseError};

mod parse_buffer;
use parse_buffer::ScheduleBuffer;

pub const DOW_MIN: u8 = 0;
pub const DOW_MAX: u8 = 6;

pub const DAY_MIN: u8 = 1;
pub const DAY_MAX: u8 = 31;

pub const MONTH_MIN: u8 = 1;
pub const MONTH_MAX: u8 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Day {
    bits: u32
}

impl Day {
    /// Create a new `Day` with no bits set.
    /// 
    /// This differs from `default`, which has all valid bits set.
    #[inline]
    pub fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Return `true` if the given bit is set.
    #[inline]
    pub fn is_set(&self, position: u8) -> bool {
        self.bits & (1u32 << position) != 0
    }

    /// Set a single day for this `Day`, from its u8 value.
    #[inline]
    pub fn set(&mut self, day: u8) -> LibResult<()> {
        if !(DAY_MIN..DAY_MAX + 1).contains(&day) {
            return Err(ScheduleParseError::InvalidValue.into());
        }

        self.bits |= 1u32 << day;

        Ok(())
    }

    /// Set a range of days for this `Day`, from a `from` and an `until` value.
    /// 
    /// Ensures that `from` is before `until`.
    pub fn set_range(&mut self, from: u8, until: u8) -> LibResult<()> {
        if from >= until {
            return Err(ScheduleParseError::BadRange.into());
        }

        let range = DAY_MIN..DAY_MAX + 1;

        if !range.contains(&from) || !range.contains(&until) {
            return Err(ScheduleParseError::InvalidValue.into());
        }

        for day in from..until + 1 {
            self.bits |= 1u32 << day;
        }

        Ok(())
    }
}

impl Default for Day {
    fn default() -> Self {
        Self {
            bits: 0xFF_FF_FF_FE
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Month{ 
    bits: u16
}

impl Month {
    pub const JAN: Self = Self { bits: 0b0000_0000_0000_0001 };
    pub const FEB: Self = Self { bits: 0b0000_0000_0000_0010 };
    pub const MAR: Self = Self { bits: 0b0000_0000_0000_0100 };
    pub const APR: Self = Self { bits: 0b0000_0000_0000_1000 };
    pub const MAY: Self = Self { bits: 0b0000_0000_0001_0000 };
    pub const JUN: Self = Self { bits: 0b0000_0000_0010_0000 };
    pub const JUL: Self = Self { bits: 0b0000_0000_0100_0000 };
    pub const AUG: Self = Self { bits: 0b0000_0000_1000_0000 };
    pub const SEP: Self = Self { bits: 0b0000_0001_0000_0000 };
    pub const OCT: Self = Self { bits: 0b0000_0010_0000_0000 };
    pub const NOV: Self = Self { bits: 0b0000_0100_0000_0000 };
    pub const DEC: Self = Self { bits: 0b0000_1000_0000_0000 };

    /// Create a new `Month` with no bits set.
    /// 
    /// This differs from `default`, which has all valid bits set.
    #[inline]
    pub fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Return `true` if the given bit is set.
    #[inline]
    pub fn is_set(&self, position: u8) -> bool {
        self.bits & (1u16 << position) != 0
    }

    /// Set a single day for this `Month`, from its u8 value.
    #[inline]
    pub fn set(&mut self, month: u8) -> LibResult<()> {
        if !(MONTH_MIN..MONTH_MAX + 1).contains(&month) {
            return Err(ScheduleParseError::InvalidValue.into());
        }

        self.bits |= 1u16 << month;

        Ok(())
    }

    /// Set a range of days for this `Month`, from a `from` and an `until` value.
    /// 
    /// Ensures that `from` is before `until`.
    pub fn set_range(&mut self, from: u8, until: u8) -> LibResult<()> {
        if from >= until {
            return Err(ScheduleParseError::BadRange.into());
        }

        let range = MONTH_MIN..MONTH_MAX + 1;

        if !range.contains(&from) || !range.contains(&until) {
            return Err(ScheduleParseError::InvalidValue.into());
        }

        for day in from..until + 1 {
            self.bits |= 1u16 << day;
        }

        Ok(())
    }
}

impl Default for Month {
    fn default() -> Self {
        Self { bits: 0x1F_FE }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DayOfWeek {
    bits: u8
}

impl DayOfWeek {
    pub const SUN: Self = Self { bits: 0b0000_0001 };
    pub const MON: Self = Self { bits: 0b0000_0010 };
    pub const TUE: Self = Self { bits: 0b0000_0100 };
    pub const WED: Self = Self { bits: 0b0000_1000 };
    pub const THU: Self = Self { bits: 0b0001_0000 };
    pub const FRI: Self = Self { bits: 0b0010_0000 };
    pub const SAT: Self = Self { bits: 0b0100_0000 };

    /// Create a new `DayOfWeek` with no bits set.
    /// 
    /// This differs from `default`, which has all valid bits set.
    #[inline]
    pub fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Return `true` if the given bit is set.
    #[inline]
    pub fn is_set(&self, position: u8) -> bool {
        self.bits & (1u8 << position) != 0
    }

    /// Set a single day for this `DayOfWeek`, from its u8 value.
    /// 
    /// It is the responsibility of the caller to guarnatee that "7" has been parsed as "0" for 
    /// Sunday.
    #[inline]
    pub fn set(&mut self, day: u8) -> LibResult<()> {
        if !(DOW_MIN..DOW_MAX + 1).contains(&day) {
            return Err(ScheduleParseError::InvalidValue.into());
        }

        self.bits |= 1 << day;

        Ok(())
    }

    /// Set a range of days for this `DayOfWeek`, from a `from` and an `until` value.
    /// 
    /// Ensures that `from` is before `until`.
    /// 
    /// It is the responsibility of the caller to guarnatee that "7" has been parsed as "0" for 
    /// Sunday.
    pub fn set_range(&mut self, from: u8, until: u8) -> LibResult<()> {
        if from >= until {
            return Err(ScheduleParseError::BadRange.into());
        }

        let range = DOW_MIN..DOW_MAX + 1;

        if !range.contains(&from) || !range.contains(&until) {
            return Err(ScheduleParseError::InvalidValue.into());
        }

        for day in from..until + 1 {
            self.bits |= 1 << day;
        }

        Ok(())
    }
}

impl Default for DayOfWeek {
    fn default() -> Self {
        Self { bits: 0b0111_1111 }
    }
}

#[derive(Debug, Default)]
pub struct Schedule {
    day: Day, // max 31, 1-indexed
    month: Month, // max 12, 1-indexed
    day_of_week: DayOfWeek // max 6, 0-indexed, on parsing 7 == 0 (Sunday)
}

impl Schedule {
    pub(super) fn set_day(&mut self, day: Day) {
        self.day = day
    }

    pub(super) fn set_month(&mut self, month: Month) {
        self.month = month
    }

    pub(super) fn set_day_of_week(&mut self, day_of_week: DayOfWeek) {
        self.day_of_week = day_of_week
    }
}

impl FromStr for Schedule {
    type Err = LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // trim, just to be nice.
        s.trim().as_bytes().try_into()
    }
}

impl TryFrom<&[u8]> for Schedule {
    type Error = LibError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let mut buf = ScheduleBuffer::new(buf);
        buf.parse()
    }
}