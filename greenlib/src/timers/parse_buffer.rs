//! Separate module to prevent accidental creation without validation of length guaranteed by the 
//! call to [`ScheduleBuffer::parse()`].
use crate::{LibError::{self, ScheduleParseError}, LibResult, errors::ScheduleParseError, timers::DEFAULT_SCHEDULE};

use super::{Day, DayOfWeek, Month, Schedule};

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
            return Err(LibError::ScheduleParseError(ScheduleParseError::TooShort));
        }

        let mut sched = Schedule::default();

        if self.buf == DEFAULT_SCHEDULE.as_bytes() {
            return Ok(sched)
        }

        sched.set_day(self.parse_day_stream()?);

        if self.cursor == self.len {
            return Err(LibError::ScheduleParseError(ScheduleParseError::TooShort));
        }
        if !self.buf[self.cursor] == b' ' {

        }

        todo!()
    }

    /// Parse a single day token value (e.g., "1", "13", etc.).
    #[inline(always)]
    fn parse_day_token(&mut self) -> LibResult<u32> {
        todo!()
    }

    /// Parse a full day stream, which can include ranges (e.g. "1-10,25").
    #[inline(always)]
    fn parse_day_stream(&mut self) -> LibResult<Day> {
        // safe ONLY because new() has validated the length. Update here if that ever changes. Test.
        if self.buf[self.cursor] == b'*' { return Ok(Day::default()) }
        
        let mut day = Day::empty();

        let mut current_int: u8 = 0;

        while self.cursor < self.len && self.buf[self.cursor] != b' ' { // Breaks at space
            if current_int == 0 {
                let val = self.buf[self.cursor].wrapping_sub(b'0');
                if !(0..10).contains(val) {
                    return Err(ScheduleParseError(ScheduleParseError::InvalidValue { field: "day", value: val }));
                }
            }
        }
    }

    #[inline(always)]
    fn parse_month_token(&mut self) -> LibResult<Month> {
        todo!()
    }

    #[inline(always)]
    fn parse_month_stream(&mut self) -> LibResult<Month> {
        todo!()
    }

    /// A single DoW token. E.g. "0", "7", "MON", "THU" etc.
    #[inline(always)]
    fn parse_day_of_week_token(&mut self) -> LibResult<u8> {
        todo!()
    }

    /// A full DoW collection. E.g. "0-2,Mon".
    fn parse_day_of_week_stream(&mut self) -> LibResult<DayOfWeek> {
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
