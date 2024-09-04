use git_conform::utils::APP_NAME;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[allow(unused_assignments)]
pub fn setup() -> Result<Vec<String>, String> {
    // Obtain the path to user's home directory,
    // the tracking file and it's contents

    let mut home_dir = String::new();
    let mut track_file_path = String::new();
    let mut track_file_contents = String::new();

    if let Some(home_path) = home::home_dir() {
        if let Some(home_path_str) = home_path.to_str() {
            home_dir = home_path_str.to_string();

            let app_data_dir = format!("{home_dir}/.local/share/{APP_NAME}");
            track_file_path = format!("{app_data_dir}/tracked");

            // Create the application data directory if one doesn't already exist
            match fs::create_dir_all(&app_data_dir) {
                Ok(()) => (),
                Err(e) => return Err(format!("{app_data_dir}: {e}"))
            };

            if let Ok(str) = fs::read_to_string(&track_file_path) {
                track_file_contents.clone_from(&str);

                // Check if the tracking file is up-to-date and remove obsolete entries if not
                for line in str.lines() {
                    if Path::new(format!("{line}/.git").as_str()).exists() {
                        if let Ok(git_status) = Command::new("git")
                            .args(["-C", line, "status"])
                            .stdout(Stdio::null())
                            .stderr(Stdio::null())
                            .status() {
                            if !git_status.success() {
                                track_file_contents = str.replace(line, "");
                            }
                        }
                        else {
                            return Err(format!("{line}: Could not execute git command"));
                        }
                    }
                    else {
                        track_file_contents = str.replace(line, "");
                    }
                }

                let mut track_file = File::create(&track_file_path).unwrap();
                match track_file.write_all(track_file_contents.as_bytes()) {
                    Ok(()) => (),
                    Err(e) => return Err(format!("{track_file_path}: {e}"))
                }
            }
        }
        else {
            return Err(String::from("Could not obtain the home directory path: the path contains invalid UTF-8 characters"));
        }
    }
    else {
        return Err(String::from("Could not find the home directory"));
    }

    Ok(Vec::from([home_dir, track_file_path, track_file_contents]))
}
