//! Initialisation steps, first activation or restart:
//!  - Any tasks to actually schedule (check for .greentimer files)?
//!     - No: Do nothing except await SIGHUP.
//!     - Yes: Current schedule set based on today's date (long term task 
//!     schedule file saved, if just restarted, will need to restart the 
//!     TimerFds, too. This must be separate one-time logic to the event 
//!     listener loop)?
//!         - No, and no failure notification set: Fetch schedule
//!         - No, and failure notification set: 
//!         - Yes: Timers 
mod dirs;
mod timers;

fn main() {
    let dirs = dirs::Dirs::from_env();
}
