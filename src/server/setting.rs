use std::fs::File;
use std::io::prelude::*;

use ron::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ServerSetting {
    pub address: String,
    pub port : u16,
    pub paths: HashMap<String, DomainPath>,
}

#[derive(Debug, Deserialize)]
pub struct DomainPath {
    pub path: String,
    pub allow: Vec<String>,
}

impl ServerSetting {
    pub fn load() -> ServerSetting {
        let mut file = match File::open("settings.ron") {
            Ok(file) => file,
            Err(err) => panic!("Failed to open settings. err:{:?}", err),
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Err(err) => panic!("Failed to read settings. err:{:?}", err),
            _ => {}
        };

        match from_str(&contents) {
            Ok(val) => return val,
            Err(err) => panic!("Failed to parse settings. err:{:?}", err),
        };
    }
}
