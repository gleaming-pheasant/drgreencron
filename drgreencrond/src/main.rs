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
use std::collections::HashSet;
use std::path::PathBuf;

use tracing::warn;

/* Internal decision, may lead to edge-case complaints, in place to prevent 
 * exhaustive path searching. MAX_DIRS is number of paths on the split, 
 * MAX_DIR_VAR_LENGTH is MAX_DIRS times a standard default max path length 
 * setting in linux. */
const MAX_DIRS: usize = 5;
const MAX_DIR_VAR_LENGTH: usize = 4096 * MAX_DIRS;
const DIR_VAR: &'static str = "GREENCRON_DIR";

// Must personally guarantee that these are valid absolute paths. No further 
// checks are done prior to runtime.
const DEFAULT_PATHS: [&'static str; 2] = ["/test123", "/test456"];

/// For whenever the system is restarted.
fn make_timer_dirs() -> HashSet<PathBuf> {
    let mut dirs: HashSet<PathBuf> = HashSet::with_capacity(
        MAX_DIRS + DEFAULT_PATHS.len()
    );

    DEFAULT_PATHS.iter().for_each(|path| { dirs.insert(PathBuf::from(path)); });
    
    if let Ok(path_list) = std::env::var(DIR_VAR) {
        if path_list.len() > MAX_DIR_VAR_LENGTH {
            warn!("{DIR_VAR} path list too long");
        } else {
            let path_split = path_list.split(':');
            
            for (ix, path_str) in path_split.enumerate() {
                
                if ix == MAX_DIRS {
                    warn!("too many dirs in {DIR_VAR}");
                    break;
                }

                let path = PathBuf::from(path_str);

                // Guard clauses ordered by cost of check for early return.
                if !path.is_absolute() {
                    warn!(path = ?path, "received non-absolute dir in {DIR_VAR}");
                    continue;
                }

                if !path.is_dir() {
                    warn!(path = ?path, "dir in {DIR_VAR} is not a directory");
                    continue;
                }

                if !path.exists() {
                    warn!(path = ?path, "dir in {DIR_VAR} doesn't exist");
                    continue;
                }

                dirs.insert(path);
            }
        }   
    }

    dirs
}

fn main() {
    let dirs = make_timer_dirs();

    for dir in dirs {
        println!("{:?}", dir);
    }
}
