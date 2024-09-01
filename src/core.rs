//! Contains the key functionality of the application

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use crate::utils::{HOME_DIR, APP_NAME, search_for_repos};

use std::path::Path;
use std::ptr::addr_of;

/// Scan specified directories only
pub fn scan_dirs(dirs: &[String]) -> Result<(), String> {
    // Directories validation

    let mut dirs_ok = true;

    for dir in dirs {
        let path = Path::new(&dir);

        // Check if the path exists
        if let Ok(p) = path.try_exists() {
            if !p {
                eprintln!("{APP_NAME}: Directory '{dir}' does not exist");
                dirs_ok = false;
                continue;
            }
        }
        else {
            eprintln!("{APP_NAME}: Cannot check the existance of directory '{dir}'");
            dirs_ok = false;
            continue;
        }

        // Check if the path leads to a file
        if path.is_file() {
            eprintln!("{APP_NAME}: '{dir}' is not a directory");
            dirs_ok = false;
        }
    }

    if !dirs_ok {
        return Err(String::from("Directories validation failed"));
    }

    search_for_repos(dirs)?;

    Ok(())
}

/// Scan all directories in user's /home
pub fn scan_all() -> Result<(), String> {
    let home_path;
    unsafe {
        home_path = addr_of!(HOME_DIR)
            .as_ref()
            .unwrap();
    }

    search_for_repos(&[home_path.to_owned()])?;

    Ok(())
}
