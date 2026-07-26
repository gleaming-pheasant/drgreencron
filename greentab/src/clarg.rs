//! Command-line arg handlers.
use greenlib::Schedule;

// Help is prioritised and returned early, even if other flags are set.

pub enum Command {
    Help,
    Process {
        command: Schedule,
        except: Vec<ExceptSchedule>
    }
}

impl Clargs {
    fn parse()
}