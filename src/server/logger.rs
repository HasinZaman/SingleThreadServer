//! logger module handles logging messages for debugging purposes
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::windows::prelude::FileExt;
use std::path::Path;

/// set_up method ensures there exists a "log.txt" for logging
/// 
/// If "log.txt" does not exist; then set_up will create a "log.txt" in the root folder
pub fn set_up() {
    if !log_file_check() {
        if let Result::Err(err) = File::create("log.txt") {
            panic!("Failed to create log.txt. {}", err);
        }
    }
}

/// log method adds message to log.txt
/// 
/// The message is in the format
/// ```text
/// --- --- ---
/// (tag)   (time_stamp)
/// (message)
/// --- --- ---
/// ```
/// 
/// # Example
/// ```
/// log("Hello", "World");
/// ```
pub fn log(tag: &str, message: String) {
    if !log_file_check() {
        set_up();
    }

    let time_stamp = chrono::offset::Utc::now();
    let line = format!(
        "\n---\t---\t---\n{}\t({})\n{}\n---\t---\t---\n",
        tag, time_stamp, message
    );

    let mut file = match OpenOptions::new().read(true).write(true).open("log.txt") {
        Err(err) => {
            panic!("Failed to access: {:?}", err);
        }
        Ok(file) => file,
    };

    let mut tmp: Vec<u8> = Vec::new();

    match file.read_to_end(&mut tmp) {
        Err(err) => {
            panic!("failed to seek to end: {:?}", err);
        }
        _ => {}
    }

    match file.seek_write(line.as_bytes(), tmp.len() as u64) {
        Err(err) => {
            panic!("failed to write at end: {:?}", err);
        }
        _ => {}
    }
}

/// log_file_check checks if log.txt exists
fn log_file_check() -> bool {
    Path::new("log.txt").exists()
}
