use toml::Value;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const DEFAULT_KEYWORD: &str = ".default.";

#[derive(Clone, Debug)]
pub struct Config {
    project: String,
    coll_name: String,
    config_file_path: String,
    primary_key: String, // Default: Collection name hash prefix + Hashed data
    data_path: String,
}

impl Config {
    pub fn new_from_path(path: &Path) -> Config {
        let config_file_path = String::from(path.to_str().unwrap());

        let mut file = File::open(path).expect("Couldn't find the configuration file!");
        let mut file_content = String::new();

        file.read_to_string(&mut file_content).unwrap();

        let parsed: Value = file_content.parse().unwrap();

        let project = match parsed["project"].as_str().unwrap() {
            DEFAULT_KEYWORD => match option_env!("CARGO_PKG_NAME") {
                Some(x) => x,
                None => panic!("Error finding project name!"),
            },
            x => x,
        }
        .to_string();

        let coll_name = match parsed["core"]["collection"].as_str().unwrap() {
            DEFAULT_KEYWORD => "main",
            x => x,
        }
        .to_string();

        let primary_key = match parsed["core"]["primary_key"].as_str().unwrap() {
            DEFAULT_KEYWORD => "k_hash",
            x => x,
        }
        .to_string();

        let data_path = match parsed["core"]["data_path"].as_str().unwrap() {
            DEFAULT_KEYWORD => format!(
                "{}/data",
                Path::new(&config_file_path)
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
            ),
            x => x.to_string(),
        };

        // TODO: Configure the last arguments
        Config {
            project,
            coll_name,
            config_file_path,
            primary_key,
            //mapped_keys_path: None,
            data_path,
        }
    }

    pub fn coll_prefix(&self) -> String {
        format!("{:x}", md5::compute(self.coll_name.clone()))
            .get(0..6)
            .unwrap()
            .to_string()
    }

    pub fn data_path(&self) -> &str {
        &self.data_path
    }

    pub fn coll_name(&self) -> &str {
        &self.coll_name
    }
}
