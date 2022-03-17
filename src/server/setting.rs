//! Setting module handles the reading and storing of server settings

use std::fs::File;
use std::io::prelude::*;

use ron::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;

/// ServerSetting is a struct that stores key information required for server start up, HTTP method handling and file retrieval
#[derive(Debug, Deserialize)]
pub struct ServerSetting {
    pub address: String,
    pub port : u16,
    pub paths: HashMap<String, DomainPath>,
}

/// DomainPath defines the path of domain in the source directory; and the extensions that can be received through GET or HEAD requests. 
#[derive(Debug, Deserialize)]
pub struct DomainPath {
    pub path: String,
    pub allow: Vec<String>,
}

impl ServerSetting {
    /// load functions read settings.ron
    /// 
    /// # return
    /// ServerSetting struct that reflect the data stored in settings.ron
    /// 
    /// #Examples
    /// ```
    /// let server_setting : ServerSetting = ServerSetting::Load();
    /// ```
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
