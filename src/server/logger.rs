use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::windows::prelude::FileExt;
use std::path::Path;

pub fn set_up() {
    if !log_file_check() {
        if let Result::Err(err) = File::create("log.txt") {
            panic!("Failed to create log.txt. {}", err);
        }
    }
}

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

fn log_file_check() -> bool {
    Path::new("log.txt").exists()
}
