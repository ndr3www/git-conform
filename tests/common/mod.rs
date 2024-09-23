use git_conform::utils::APP_NAME;

use std::fs::{self, File};
use std::process::{Command, Stdio};

#[allow(unused_assignments)]
pub fn setup() -> Result<(String, String, String), String> {
    // Obtain the path to user's home directory,
    // the tracking file and it's contents

    let mut home_dir = String::new();
    let mut track_file_path = String::new();

    let mut tests_dir = String::new();

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

            tests_dir = format!("{app_data_dir}/tests");
            
            // Create dummy repositories and files for testing
            for n in 1..=3 {
                let real_no_hidden = format!("{tests_dir}/repo{n}");
                let fake_no_hidden = format!("{tests_dir}/fake_repo{n}/.git");
                let dir_no_hidden = format!("{tests_dir}/dir{n}");

                let real_hidden = format!("{tests_dir}/.hidden/repo{n}");
                let fake_hidden = format!("{tests_dir}/.hidden/fake_repo{n}/.git");
                let dir_hidden = format!("{tests_dir}/.hidden/dir{n}");

                let file = format!("{tests_dir}/file{n}");

                // Real, no hidden
                match fs::create_dir_all(&real_no_hidden) {
                    Ok(()) => (),
                    Err(e) => return Err(format!("{real_no_hidden}: {e}"))
                };
                Command::new("git")
                    .args(["-C", real_no_hidden.as_str(), "init"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .map_err(|e| format!("{real_no_hidden}: {e}"))?;

                // Fake, no hidden
                match fs::create_dir_all(&fake_no_hidden) {
                    Ok(()) => (),
                    Err(e) => return Err(format!("{fake_no_hidden}: {e}"))
                };

                // Regular directory, no hidden
                match fs::create_dir_all(&dir_no_hidden) {
                    Ok(()) => (),
                    Err(e) => return Err(format!("{dir_no_hidden}: {e}"))
                };

                // Real, hidden
                match fs::create_dir_all(&real_hidden) {
                    Ok(()) => (),
                    Err(e) => return Err(format!("{real_hidden}: {e}"))
                };
                Command::new("git")
                    .args(["-C", real_hidden.as_str(), "init"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .map_err(|e| format!("{real_hidden}: {e}"))?;

                // Fake, hidden
                match fs::create_dir_all(&fake_hidden) {
                    Ok(()) => (),
                    Err(e) => return Err(format!("{fake_hidden}: {e}"))
                };

                // Regular directory, hidden
                match fs::create_dir_all(&dir_hidden) {
                    Ok(()) => (),
                    Err(e) => return Err(format!("{dir_hidden}: {e}"))
                };

                // Files
                match File::create(&file) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("{file}: {e}"))
                };
            }
        }
        else {
            return Err(String::from("Could not obtain the home directory path: the path contains invalid UTF-8 characters"));
        }
    }
    else {
        return Err(String::from("Could not find the home directory"));
    }

    Ok((home_dir, track_file_path, tests_dir))
}
