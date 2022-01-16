use pathsearch::find_executable_in_path;
use std::path::PathBuf;

pub mod cli;
pub mod error;

pub fn find_daemon_executable() -> Option<PathBuf> {
    find_executable_in_path("mediarepo-daemon")
}
