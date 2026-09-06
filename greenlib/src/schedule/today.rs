//! Everything within this module works around UTC.
//! 
//! If ever offsets (including BST) are required, implement them in parsing, and still use UTC here.
use std::time::{SystemTime, UNIX_EPOCH};


/// Contains date information matching that in the schedule; day of the month, month and day of the 
/// week.
pub struct Date {
    day: u8,
    month: u8,
    day_of_week: u8
}

impl Date {
    /// Fetch the current day of the week from system time.
    pub fn fetch() -> Self {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time has gone backwards...")
            .as_secs() as i64;
        
        let days_since_epoch = secs / 86_400;
        let mut day_of_week = ((days_since_epoch + 4) % 7 + 7) % 7;
        if day_of_week == 0 { day_of_week = 7; }

        let z = days_since_epoch + 719_468;
        let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
        let doe = (z - era * 146_097) as u32;
        let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
        let y = (yoe as i64) + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;

        let day = doy - (153 * mp + 2) / 5 + 1;         // 1..31
        let month = if mp < 10 { mp + 3 } else { mp - 9 }; // 1..12
        let _year = y + if month <= 2 { 1 } else { 0 };

        Self {
            day: day as u8,
            month: month as u8,
            day_of_week: day_of_week as u8
        }
    }
}