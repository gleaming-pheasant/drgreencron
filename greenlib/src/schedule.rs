use std::str::FromStr;

use crate::DayOfMonth;
use crate::errors::Error;

/// All fields are u64, even where the CronField is just DOW or month (u8 and 
/// u16 respectively). Adds 24 bytes per `CronSchedule`, but for the cleaner 
/// code and smaller program build, this is a reasonable tradeoff.
#[derive(Debug)]
pub struct CronField(u64);

impl CronField {

}

/// Simplified version of the crontab scheduler. As the whole purpose of this 
/// app is to find the best time of day to run a scheduled task, we only really 
/// need to know which days to run it on (see [`ExceptSchedule`] for a way to 
/// restrict the task scheduler to not run at certain times, including hour and 
/// minute).
#[derive(Debug)]
pub struct Schedule {
    dom: DayOfMonth,
    month: CronField, // max 11
    dow: CronField // max 6
}

impl FromStr for Schedule {
    type Err = crate::errors::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // first check correct num components, otherwise, can't tell intent for 
        // fields.
        let parts: Vec<&str> = s.split(' ').collect();
        // dom, month, dow
        if parts.len() != 3 { 
            return Err(Error::ParseError { 
                kind: ParseErrorKind::Schedule, received: s.to_owned()
            }) 
        }

        // safe index calls, already confirmed 3 parts.
        let doms = "-"
    }
}

#[derive(Debug)]
pub struct ExceptSchedule {
    minute: CronField, // max 59
    hour: CronField, // max 23
    dom: CronField, // max 31
    month: CronField, // max 11
    dow: CronField, // max 6
}