use crate::ken_all::database::DatabaseConfig;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

impl Config {
    pub fn read_file(path: String) -> Result<String, std::io::Error> {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();

        reader.read_to_string(&mut content)?;

        Ok(content)
    }
    fn string_to_config(content: String) -> Result<Config, toml::de::Error> {
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }

    pub fn read(path: String) -> Config {
        let content = Config::read_file(path).expect("Failed to read a config!");
        let config = Config::string_to_config(content).expect("Failed to parse a config!");

        config
    }
}
