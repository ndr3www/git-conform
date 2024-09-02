use git_conform::utils::{
    HOME_DIR,
    APP_NAME,
    APP_DATA_DIR,
    APP_TRACK_FILE_PATH,
    APP_TRACK_FILE,
    handle_error
};

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::ptr::addr_of;
use std::process::{Command, Stdio};

pub fn setup() {
    // Obtain the path to user's home directory,
    // the application data directory and the tracking file
    // Also read the tracking file contents
    if let Some(home_path) = home::home_dir() {
        if let Some(home_path_str) = home_path.to_str() {
            unsafe {
                HOME_DIR = home_path_str.to_string();

                APP_DATA_DIR = format!("{HOME_DIR}/.local/share/{APP_NAME}");
                APP_TRACK_FILE_PATH = format!("{APP_DATA_DIR}/tracked");

                if let Ok(str) = fs::read_to_string(addr_of!(APP_TRACK_FILE_PATH).as_ref().unwrap()) {
                    APP_TRACK_FILE.clone_from(&str);

                    // Check if the tracking file is up-to-date and remove obsolete entries if not

                    for line in str.lines() {
                        if Path::new(format!("{line}/.git").as_str()).exists() {
                            if let Ok(git_status) = Command::new("git")
                                .args(["-C", line, "status"])
                                .stdout(Stdio::null())
                                .stderr(Stdio::null())
                                .status() {
                                if !git_status.success() {
                                    APP_TRACK_FILE = str.replace(line, "");
                                }
                            }
                            else {
                                handle_error(format!("{line}: Could not execute git command").as_str(), 1);
                            }
                        }
                        else {
                            APP_TRACK_FILE = str.replace(line, "");
                        }
                    }

                    let mut track_file = File::create(
                        addr_of!(APP_TRACK_FILE_PATH)
                            .as_ref()
                            .unwrap())
                        .unwrap();
                    match track_file.write_all(APP_TRACK_FILE.as_bytes()) {
                        Ok(()) => (),
                        Err(e) => handle_error(format!("{APP_TRACK_FILE_PATH}: {e}").as_str(), 1)
                    }
                }
            }
        }
        else {
            handle_error("Could not obtain the home directory path: the path contains invalid UTF-8 characters", 1);
        }
    }
    else {
        handle_error("Could not find the home directory", 1);
    }
}
