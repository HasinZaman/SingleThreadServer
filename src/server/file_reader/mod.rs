use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

pub fn parse(url: &str, search_folder: &str, allowed_extension: &Vec<String>) -> Option<PathBuf> {
    let url = url.trim_matches('\\').trim_matches('/');

    let mut path_buffer = PathBuf::new();

    path_buffer.push(r"Site");

    path_buffer.push(&search_folder);

    match url.rfind('?') {
        Some(index) => path_buffer.push(url[0..index].trim_matches('\\')),
        None => path_buffer.push(url.trim_matches('\\')),
    }

    if path_buffer.extension().is_none() {
        path_buffer.push("index.html");
    }

    let extension = match path_buffer.extension() {
        Some(extension) => extension.to_str().unwrap(),
        None => panic!("path_buffer has no file extension"),
    };

    let valid_extension = allowed_extension.iter().any(|e| e == extension);

    if path_buffer.exists() && valid_extension {
        return Option::Some(path_buffer);
    }

    return Option::None;
}

pub fn get_file_content_string(file_path: &Path) -> Option<String> {
    let mut file = match File::open(file_path) {
        Err(_err) => return Option::None,
        Ok(file) => file,
    };

    let mut contents: String = String::new();

    match file.read_to_string(&mut contents) {
        Err(_err) => return Option::None,
        Ok(_) => return Option::Some(contents),
    }
}

pub fn get_file_content_bytes(file_path: &Path) -> Option<Vec<u8>> {
    let mut file = match File::open(file_path) {
        Err(_err) => return Option::None,
        Ok(file) => file,
    };

    let mut contents: Vec<u8> = Vec::new();

    match file.read_to_end(&mut contents) {
        Err(_err) => return Option::None,
        Ok(_) => return Option::Some(contents),
    }
}
