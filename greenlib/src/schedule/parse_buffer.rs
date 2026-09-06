//! Separate module to prevent accidental creation without validation of length guaranteed by the 
//! call to [`ScheduleBuffer::parse()`].
use crate::schedule::MONTH_MAX;
use crate::{LibResult, schedule::DAY_MAX};
use crate::errors::ScheduleParseError;

use super::{Day, DayOfWeek, Month, Schedule};

pub const DEFAULT_SCHEDULE: &'static str = "* * *";
const MAX_SCHEDULE_LEN: usize = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,\
26,27,28,29,30,31 JAN,FEB,MAR,APR,MAY,JUN,JUL,AUG,SEP,OCT,NOV,DEC SUN,MON,TUE,WED,THU,FRI,SAT".len();

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
    #[inline]
    pub(super) fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            cursor: 0,
            len: buf.len()
        }
    }

    /// Attempt the parse the contained buffer as a [`Schedule`].
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

                    if current_val > DAY_MAX {
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

    fn parse_month(&mut self) -> LibResult<Month> {
        // safe ONLY because new() has validated the length. Update here if that ever changes. Test.
        if self.buf[self.cursor] == b'*' {
            self.cursor += 1;
            return Ok(Month::default())
        }

        let mut month = Month::empty();
        let mut current_val: u8 = 0;
        let mut range_start: Option<u8> = None;
        let mut parsed_any = false;


        while self.cursor < self.len && self.buf[self.cursor] != b' ' {
            let b = self.buf[self.cursor];

            match b {
                b'a'..=b'z' | b'A'..=b'Z' => {
                    if self.len - self.cursor >= 3 {
                        let mut lower_digits = [0u8; 3];
                        // to lower case if needed.
                        self.buf[self.cursor..self.cursor + 3].iter().enumerate()
                            .for_each(|(digit, value)| {
                                lower_digits[digit] = if value < &b'a' { value + 32 } else { *value };
                            });

                        current_val = match &lower_digits {
                            b"jan" => 1,
                            b"feb" => 2,
                            b"mar" => 3,
                            b"apr" => 4,
                            b"may" => 5,
                            b"jun" => 6,
                            b"jul" => 7,
                            b"aug" => 8,
                            b"sep" => 9,
                            b"oct" => 10,
                            b"nov" => 11,
                            b"dec" => 12,
                            _ => return Err(ScheduleParseError::InvalidMonth.into())
                        };

                        self.cursor += 3;
                    } else {
                        return Err(ScheduleParseError::InvalidMonth.into())
                    }
                },
                b'0'..=b'9' => {
                    let digit = b.wrapping_sub(b'0');
                    current_val = current_val.saturating_mul(10).saturating_add(digit);

                    if current_val > MONTH_MAX {
                        return Err(ScheduleParseError::InvalidValue.into());
                    }

                    self.cursor += 1;
                },
                b'-' => {
                    if current_val == 0 || range_start.is_some() {
                        return Err(ScheduleParseError::InvalidDigit.into());
                    }

                    range_start = Some(current_val);
                    current_val = 0;
                    self.cursor += 1;
                },
                b',' => {
                    if current_val == 0 {
                        return Err(ScheduleParseError::InvalidDigit.into());
                    }
                    if let Some(start) = range_start { // range ending
                        month.set_range(start, current_val)?;
                        range_start = None;
                    } else {
                        month.set(current_val)?;
                    }

                    current_val = 0;
                    parsed_any = true;
                    self.cursor += 1;
                },
                _ => return Err(ScheduleParseError::InvalidDigit.into())
            }
        }

        // reached the end of "month", process remaining val.
        if current_val != 0 {
            if let Some(start) = range_start {
                month.set_range(start, current_val)?;
            } else {
                month.set(current_val)?;
            }
            parsed_any = true;
        }

        // received nothing to parse
        if !parsed_any {
            return Err(ScheduleParseError::InvalidValue.into())
        }

        Ok(month)
    }

    /// A full DoW collection. E.g. "0-2,Mon".
    fn parse_day_of_week(&mut self) -> LibResult<DayOfWeek> {
        // safe ONLY because new() has validated the length. Update here if that ever changes. Test.
        if self.buf[self.cursor] == b'*' {
            self.cursor += 1;
            return Ok(DayOfWeek::default())
        }

        let mut dow = DayOfWeek::empty();
        let mut current_val: u8 = 0;
        let mut range_start: Option<u8> = None;
        let mut parsed_any = false;


        while self.cursor < self.len && self.buf[self.cursor] != b' ' {
            let b = self.buf[self.cursor];

            match b {
                b'a'..=b'z' | b'A'..=b'Z' => {
                    if self.len - self.cursor >= 3 {
                        let mut lower_digits = [0u8; 3];
                        // to lower case if needed.
                        self.buf[self.cursor..self.cursor + 3].iter().enumerate()
                            .for_each(|(digit, value)| {
                                lower_digits[digit] = if value < &b'a' { value + 32 } else { *value };
                            });

                        current_val = match &lower_digits {
                            b"mon" => 1,
                            b"tue" => 2,
                            b"wed" => 3,
                            b"thu" => 4,
                            b"fri" => 5,
                            b"sat" => 6,
                            b"sun" => 7,
                            _ => return Err(ScheduleParseError::InvalidDayOfWeek.into())
                        };

                        self.cursor += 3;
                    } else {
                        return Err(ScheduleParseError::InvalidDayOfWeek.into())
                    }
                },
                b'0' | b'7' => {
                    current_val = 7;
                    self.cursor += 1;
                },
                b'1' => { current_val = 1; self.cursor += 1 },
                b'2' => { current_val = 2; self.cursor += 1 },
                b'3' => { current_val = 3; self.cursor += 1 },
                b'4' => { current_val = 4; self.cursor += 1 },
                b'5' => { current_val = 5; self.cursor += 1 },
                b'6' => { current_val = 6; self.cursor += 1 },
                b'-' => {
                    if current_val == 0 || range_start.is_some() {
                        return Err(ScheduleParseError::InvalidDigit.into());
                    }

                    range_start = Some(current_val);
                    current_val = 0;
                    self.cursor += 1;
                },
                b',' => {
                    if current_val == 0 {
                        return Err(ScheduleParseError::InvalidDigit.into());
                    }
                    if let Some(start) = range_start { // range ending
                        dow.set_range(start, current_val)?;
                        range_start = None;
                    } else {
                        dow.set(current_val)?;
                    }

                    current_val = 0;
                    parsed_any = true;
                    self.cursor += 1;
                },
                _ => return Err(ScheduleParseError::InvalidDigit.into())
            }
        }

        // reached the end of "dow", process remaining val.
        if current_val != 0 {
            if let Some(start) = range_start {
                dow.set_range(start, current_val)?;
            } else {
                dow.set(current_val)?;
            }
            parsed_any = true;
        }

        // received nothing to parse
        if !parsed_any {
            return Err(ScheduleParseError::InvalidValue.into())
        }

        Ok(dow)
    }
}