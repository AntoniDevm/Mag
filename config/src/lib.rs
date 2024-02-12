use std::fs;
use serde::{Deserialize, Serialize};
use std::io::Error;
use std::io::ErrorKind;

#[derive(Serialize, Deserialize)]
pub struct Config {}

impl Config {
    pub fn new(file: &str) -> std::io::Result<Config> {
        let yaml = match fs::read_to_string(file) {
            Ok(y) => y,
            Err(er) => {
                log::error!("Error reading file");
                log::debug!("Error Message: {}",er);
                return Err(er);

            }
        };
        let config = match serde_yaml::from_str(&yaml) {
            Ok(c) => c,
            Err(er) => {
                log::error!("Error parsing config file");
                log::debug!("Error Message: {}",er);
                return Err(Error::new(ErrorKind::InvalidInput, er)); 
            }
        };
        Ok(config)
    }
}
