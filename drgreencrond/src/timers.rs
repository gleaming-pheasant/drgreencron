use std::path::PathBuf;

#[derive(Debug)]
pub struct GreenTimer {
    path: PathBuf
}

impl From<PathBuf> for GreenTimer {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}