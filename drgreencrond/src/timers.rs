#[derive(Debug)]
pub struct Schedule {
    day_of_month: u32,
    month: u16,
    day_of_week: u8
}


#[derive(Debug)]
pub struct Except {
    minute: u64,
    hour: u32,
    day_of_month: u32,
    month: u16,
    day_of_week: u8
}


#[derive(Debug)]
pub struct GreenTimer {
    name: String,
    schedule: Schedule,
    except: Except
}

impl GreenTimer {
    fn new(name: String) -> anyhow::Result<Self> {
        let schedule = Self::get_schedule_from_path(&name);
        let except = Self::get_except_from_path(&name);

        let new_timer = Self {
            name,
            schedule,
            except
        };

        if !new_timer.has_collisions() {
            Ok(new_timer)
        } else {
            Err(anyhow::Error::msg("Collision detected."))
        }        
    }

    /// Compare potential collision fields between schedule and except. For 
    /// example, if schedule has 00111110 (Monday-Friday), but except has 
    /// minute = 1152921504606846975 (all set, this is 60 least significant 
    /// bits all set - 4 most significant bits unset for check, meaning not at 
    /// any minute) and hour = 16777215 (24 least significant bits set - 8 
    /// most significant bit unset for check - meaning all 24 hours), but there 
    /// is a match on AND for the DoM, Month or DoW for the schedule, then the 
    /// schedule cannot run.
    /// 
    /// Use this for user feedback in greentab, too.
    fn has_collisions(&self) -> bool {
        todo!()
    }

    // Deserialize schedule text from the greentimer file.
    fn get_schedule_from_path(name: &str) -> Schedule {
        todo!()
    }

    // Deserialize except text from the greentimer file. These three functions 
    // should be in a shared crate, for reuse in the daemon and greentab.
    fn get_except_from_path(name: &str) -> Except {
        todo!()
    }
}