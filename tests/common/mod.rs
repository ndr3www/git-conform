use git_conform::utils::{
    HOME_DIR,
    APP_NAME,
    APP_DATA_DIR,
    APP_TRACK_FILE_PATH,
    APP_TRACK_FILE,
    handle_error
};

use std::fs;
use std::ptr::addr_of;

pub fn setup() {
    // Obtain user's home directory path and use it for
    // creating a full path to the application data directory,
    // tracking file and reading the tracking file contents
    if let Some(home_path) = home::home_dir() {
        if let Some(home_path_str) = home_path.to_str() {
            unsafe {
                HOME_DIR = home_path_str.to_string();

                APP_DATA_DIR = format!("{HOME_DIR}/.local/share/{APP_NAME}");
                APP_TRACK_FILE_PATH = format!("{APP_DATA_DIR}/tracked");

                if let Ok(s) = fs::read_to_string(addr_of!(APP_TRACK_FILE_PATH).as_ref().unwrap()) {
                    APP_TRACK_FILE = s;
                }
            }
        }
        else {
            handle_error("Home directory path contains invalid UTF-8 characters", 1);
        }
    }
    else {
        handle_error("Could not find home directory", 1);
    }
}
