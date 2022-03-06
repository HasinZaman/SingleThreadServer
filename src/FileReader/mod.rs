use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn parse(url : &str) -> PathBuf {
    let path_buffer = url_to_PathBuf(url);

    let path = path_buffer.as_path();
    
    if path.exists() {
        return path_buffer
    }

    let path_buffer = PathBuf::from("404.html");

    path_buffer
}

fn url_to_PathBuf(url : &str) -> PathBuf{
    let last_slash = url.rfind('\\');
    let last_dot = url.rfind('.');
    let last_question_mark = url.rfind('?');

    let url_end : usize;

    match last_question_mark {
        Some(index) => url_end = index - 1,
        None => url_end = url.len(),
    }

    match last_slash {
        None => {
            return PathBuf::from("index.html");
        },
        _ => {},
    }
    let last_slash = last_slash.unwrap();
    let url_path : &str = &url[0..last_slash - 1];

    match last_dot {
        None => {
            return PathBuf::from(format!("{}\\index.html", url_path));
        },
        _ => {},
    }
    let last_dot = last_dot.unwrap();
    let url_file_name : &str = &url[last_slash + 1..last_dot-1];

    let url_extension : &str = &url[last_dot + 1..url_end];

    return PathBuf::from(format!("{}\\{}.{}", url_path, url_file_name, url_extension));
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