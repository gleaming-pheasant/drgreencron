//! Separate module to prevent accidental creation without validation of length guaranteed by the 
//! call to [`ScheduleBuffer::parse()`].
use crate::LibResult;
use crate::errors::ScheduleParseError;

use super::{Day, DayOfWeek, Month, Schedule};

pub const DEFAULT_SCHEDULE: &'static str = "* * *";
const MAX_SCHEDULE_LEN: usize =
    30 + // commas for 31 days
    9 + (22 * 2) + // digits for days, 1-9 + double digits up to 31
    11 + // commas for 12 months
    (3 * 12) + // 3-char months
    6 + // commas for days of week
    (3 * 6); // 3-char DoWs.

enum RangeState {
    Hold(u8),

}

/// Exists simply to make the code easier to follow and prevent passing around buffer and cursor to 
/// parse separate components.
/// 
/// __Warning!__ Beware when updating the `parse` function, that the functions for parsing 
/// component parts of a Schedule rely on `parse` validating the current cursor index can be safely 
/// read and is in the correct position to be the start of the component value. These functions MUST 
/// be called in order.
#[derive(Debug)]
pub(super) struct ScheduleBuffer<'a> {
    buf: &'a [u8],
    cursor: usize,
    len: usize
}

impl<'a> ScheduleBuffer<'a> {
    /// Create a new `ScheduleBuffer` from the provided buffer.
    #[inline(always)]
    pub(super) fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            cursor: 0,
            len: buf.len()
        }
    }

    /// Attempt the parse the contained buffer as a [`Schedule`].
    #[inline(always)]
    pub(super) fn parse(&mut self) -> LibResult<Schedule> {
        if self.len < DEFAULT_SCHEDULE.len() {
            return Err(ScheduleParseError::TooShort.into());
        }

        // prevent infinite parsing of valid values. parse individual components doesn't actually 
        // verify that a bit has already been set, so a malicious function could call 
        // "1,1,1,1,1,1,1..." indefinitely.
        if self.len > MAX_SCHEDULE_LEN {
            return Err(ScheduleParseError::TooLong.into());
        }

        let mut sched = Schedule::default();

        if self.buf == DEFAULT_SCHEDULE.as_bytes() {
            return Ok(sched)
        }

        sched.set_day(self.parse_day()?);

        // individual component parsing functions will guarantee that if this isn't true, it will be 
        // a space " " and not the end.
        if self.cursor == self.len {
            return Err(ScheduleParseError::TooShort.into());
        }
        
        self.cursor += 1;

        sched.set_month(self.parse_month()?);

        if self.cursor == self.len {
            return Err(ScheduleParseError::TooShort.into());
        }

        self.cursor += 1;
        sched.set_day_of_week(self.parse_day_of_week()?);
        // don't need to check end, parse_day_of_week will just keep running until it encounters an 
        // error.
        
        Ok(sched)
    }

    /// Parse a full day stream, which can include ranges (e.g. "1-10,25").
    #[inline(always)]
    fn parse_day(&mut self) -> LibResult<Day> {
        // safe ONLY because new() has validated the length. Update here if that ever changes. Test.
        if self.buf[self.cursor] == b'*' {
            self.cursor += 1;
            return Ok(Day::default())
        }
        
        let mut day = Day::empty();
        let mut current_val: u8 = 0;
        let mut range_start: Option<u8> = None;
        let mut parsed_any = false;

        while self.cursor < self.len && self.buf[self.cursor] != b' ' {
            let b = self.buf[self.cursor];

            match b {
                b'0'..=b'9' => {
                    let digit = b.wrapping_sub(b'0');
                    current_val = current_val.saturating_mul(10).saturating_add(digit);

                    if current_val > 31 {
                        return Err(ScheduleParseError::InvalidValue.into());
                    }
                },
                b'-' => {
                    if current_val == 0 || range_start.is_some() {
                        return Err(ScheduleParseError::InvalidDigit.into());
                    }

                    range_start = Some(current_val);
                    current_val = 0;
                },
                b',' => {
                    if current_val == 0 {
                        return Err(ScheduleParseError::InvalidDigit.into());
                    }
                    if let Some(start) = range_start { // range ending
                        day.set_range(start, current_val)?;
                        range_start = None;
                    } else {
                        day.set(current_val)?;
                    }

                    current_val = 0;
                    parsed_any = true;
                },
                _ => return Err(ScheduleParseError::InvalidDigit.into())
            }
            self.cursor += 1;
        }

        // reached the end of "day", process remaining val.
        if current_val > 0 {
            if let Some(start) = range_start {
                day.set_range(start, current_val)?;
            } else {
                day.set(current_val)?;
            }
            parsed_any = true;
        }

        // received nothing to parse
        if !parsed_any {
            return Err(ScheduleParseError::InvalidValue.into())
        }

        Ok(day)
    }

    #[inline(always)]
    fn parse_month(&mut self) -> LibResult<Month> {
        todo!()
    }

    /// A full DoW collection. E.g. "0-2,Mon".
    fn parse_day_of_week(&mut self) -> LibResult<DayOfWeek> {
        if self.buf[self.cursor] == b'*' { return Ok(DayOfWeek::default()) }

        while self.cursor < self.len {
            
            self.cursor += 1;
        }

        let mut dow = DayOfWeek::empty();

        // only need to attempt one digit or three alpha bytes for DoW; can only be short name token 
        // or 0-7.
        if self.buf[self.cursor].is_ascii_digit() {
            let val = self.buf[self.cursor].wrapping_sub(b'0');
            
        }

        todo!()
    }
}
