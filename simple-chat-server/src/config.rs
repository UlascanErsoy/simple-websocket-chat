use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
 pub host: String,
 pub port: String,
 pub server_name: String
}

impl ServerConfig {

    pub fn from_file(path: &str) -> ServerConfig {
        let file = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => panic!("Could not read file! {e}")
        };
        
        let config: ServerConfig = match serde_yaml::from_str(&file) {
            Ok(res) => res,
            Err(e) => panic!("Could not parse YAML! {e}")
        };
        
        config
    }

}
