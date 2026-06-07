use std::ffi::OsString;
use std::path::PathBuf;

use tracing::warn;

use crate::timers::GreenTimer;

const DIR_VAR: &'static str = "GREENCRON_DIR";

/// In place to prevent exhaustive path searching.This is a standard default max 
/// absolute path length setting in linux systems. */
const MAX_DIR_VAR_LENGTH: usize = 4096;

// Must guarantee this is valid at compile time!
const DEFAULT_DIR: &'static str = concat!("/etc/", env!("CARGO_PKG_NAME"));

#[derive(Debug)]
pub struct Dirs {
    dirs: Vec<SearchDir>
}

impl Dirs {
    pub fn from_env() -> Self {
        let mut search_dirs = vec![SearchDir::from(PathBuf::from(DEFAULT_DIR))];

        if let Ok(dir) = std::env::var(DIR_VAR) {
            // Track issues verbosely, rather than short circuiting on single 
            // check.
            let mut fault = false;

            let custom_search_dir = PathBuf::from(dir);

            // Guard clauses ordered by cost of check for early return.
            if !custom_search_dir.is_absolute() {
                warn!(custom_search_dir = %custom_search_dir.display(), "received non-absolute dir in {DIR_VAR}");
                fault = false;
            }

            if !custom_search_dir.is_dir() {
                warn!(custom_search_dir = %custom_search_dir.display(), "dir in {DIR_VAR} is not a directory");
                fault = false;
            }

            if !custom_search_dir.exists() {
                warn!(custom_search_dir = %custom_search_dir.display(), "dir in {DIR_VAR} doesn't exist");
                fault = false;
            }

            if !fault {
                search_dirs.push(SearchDir::from(custom_search_dir));
            }
        }

        Self::from(search_dirs)
    }
}

impl From<Vec<SearchDir>> for Dirs {
    fn from(dirs: Vec<SearchDir>) -> Self {
        Self { dirs }
    }
}

#[derive(Debug)]
pub struct SearchDir {
    path: PathBuf,
    timers: Vec<GreenTimer>
}

impl From<PathBuf> for SearchDir {
    /// Read all of the contained .grntimer files in the provided `PathBuf` and 
    /// add them to the list of `timers`. Assumes the path is present and is a 
    /// directory, though will only silently fail if it is not.
    fn from(path: PathBuf) -> Self {
        match path.read_dir() {
            Ok(read_dir) => {
                let timers = read_dir.into_iter().filter_map(|entry| {
                    entry.ok().map(|entry| {
                        if entry.path().extension() == Some(&OsString::from("grntimer")) {
                            GreenTimer::from(entry.path())
                        }
                    })
                }).collect();

                Self { path, timers }
            },
            Err(e) => {
                // Exit loud, even if custom path, user should be made aware.
                warn!(path = %path.display(), "could not read timer files path");
                std::process::exit(1);
            }
        }
    }
}