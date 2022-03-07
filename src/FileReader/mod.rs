use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

pub fn parse(url : &str) -> PathBuf {
    let path_buffer = url_to_path_buffer(url);

    let path = path_buffer.as_path();
    
    if path.exists() {
        return path_buffer
    }

    let path_buffer = PathBuf::from("Site\\404.html");

    path_buffer
}

fn url_to_path_buffer(url : &str) -> PathBuf{
    let url = url.trim_start_matches('\\');

    let last_slash = url.rfind('\\');
    let last_dot = url.rfind('.');
    let last_question_mark = url.rfind('?');

    let url_end : usize;

    let file_name : &str;
    let extension : &str;
    let path : &str;

    match last_question_mark {
        Some(index) => {
            url_end = index;
        },
        None => url_end = url.len(),
    }

    match last_slash {
        None => {
            path = "";
        },
        Some(index) => {
            path = &url[0..index].trim_matches('\\');
        },
    }

    //todo
    // check for invalid paths

    match last_dot {
        None => {
            return PathBuf::from(format!("Site\\{}\\index.html", path));
        },
        Some(index) => {
            file_name = &url[path.len()..index].trim_start_matches('\\').trim_end_matches('.');
            extension = &url[index..url_end].trim_start_matches('.');
        },
    }

    //todo
    // check for file names
    // check for invalid extensions

    return PathBuf::from(
        format!(
            "Site\\{}\\{}.{}",
            path,
            file_name,
            extension
        )
    );
}

pub fn get_file_content(file_path : &Path) -> Option<String> {
    let mut file = match File::open(file_path) {
        Err(err) => return Option::None,
        Ok(file) => file,
    };

    let mut contents : String = String::new();

    match file.read_to_string(&mut contents) {
        Err(err) => return Option::None,
        Ok(_) => return Option::Some(contents),
    }
}