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
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod dirs;
mod timers;

fn main() {
    let registry = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_target(false));
    
    match tracing_journald::layer() {
        Ok(layer) => {
            registry.with(layer).init();
        },
        Err(e) => {
            // Typically to stdio if can't send traces to journald
            registry.init();
            tracing::error!("couldn't connect to journald: {}", e);
        }
    }

    let dirs = dirs::Dirs::from_env();
}
