//! General utilities

#![allow(unused_imports)]
use std::{
    env,
    path::{Path, PathBuf},
    time::SystemTime,
};

mod clamp;
pub use self::clamp::*;

/// Return string representation of elapsed time
pub fn elapsed(start_time: &SystemTime) -> String {
    let elapsed = start_time.elapsed().unwrap();
    format!(
        "{}s {:.*}ms",
        elapsed.as_secs(),
        1,
        elapsed.subsec_nanos() as f64 / 1_000_000.0
    )
}

/// Return elapsed time in seconds
pub fn time(start_time: &SystemTime) -> f32 {
    start_time.elapsed().unwrap().as_millis() as f32 / 1000.0
}

/// Detect framework assets only
pub fn assets_library(value: &str) -> PathBuf {
    let path = PathBuf::from(value);
    if path.is_relative() {
        // deal with relative paths
        if cfg!(debug_assertions) {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join(path)
        } else {
            match env::current_exe().unwrap().parent() {
                Some(dir) => dir.join("assets").join(path),
                None => {
                    // exe located in root. aka "/"
                    path
                }
            }
        }
    } else {
        path
    }
}
