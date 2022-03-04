use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_file_path(url : &str) -> &Path {
    //split website & URL variables
    let url_parts = url.split("?");

    //get url portion of request
    let mut file_name = match url_parts.next() {
        None => panic!()/*There should exist a string*/,
        Some(file_name) => url, 
    };

    let mut path : &Path = Path::new(format!("/Site/{}", file_name));

    if path.extension().is_none() {
        path = Path::new(format!("/Site/{}/index.html", file_name));
    }

    if !path.exists() {
        path = Path::new(format!("/Site/404.html"));
    }

    return path;
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