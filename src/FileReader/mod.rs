use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn parse(url : &str) -> PathBuf{
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
    
    /*let mut url_char : char;
    let mut tmp_index : usize = 0;
    let mut i1 : usize = 0;

    let mut url_iter = url.chars().into_iter().map(|x|{
        url_char = x;
        x
    });

    let define_range = |start : usize, checkpoint_char : Option<char>, end_char : Option<char>| -> ([usize; 2], bool) {
        let range : [usize; 2] = [start, 0];
        let mut end_cond : bool = true;

        while url_iter.next().is_some() {
            let tmp = Option::Some(url_char);

            if tmp == checkpoint_char {
                tmp_index = i1;
            }
            if tmp == end_char {
                range[1] = tmp_index;
                end_cond = false;
            }
            i1+=1;
        }

        if end_cond {
            range[1] = i1;
        }

        (range, end_cond)
    };

    let (path_range, file_name_exist) = define_range(0, Option::Some('\\'), Option::Some('.'));

    if file_name_exist {
        return PathBuf::from(
            format!(
                "{}\\index.html",
                url[path_range[0]..path_range[1]]
            )
        )
    }

    let (name_range, extension_exist) = define_range(path_range[1] + 1, Option::Some('.'), Option::Some('.'));

    let (extension_range, extension_exist) = define_range(name_range[1] + 1, Option::None, Option::Some('?'));

    return PathBuf::from(
        format!(
            "{}\\{}{}",
            url[path_range[0]..path_range[1]],
            url[name_range[0]..name_range[1]],
            url[extension_range[0]..extension_range[1]]
        )
    );
    */
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