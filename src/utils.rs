//! Custom helper utilities

use std::process;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub static mut APP_DATA_DIR: String = String::new();

/// Prints given error message to the standard error with application name and then exits the application with specified error code
pub fn handle_error(error: &str, code: i32) {
    eprintln!("{APP_NAME}: {error}");
    process::exit(code);
}
