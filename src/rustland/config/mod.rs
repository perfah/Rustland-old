use std::path::PathBuf;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::env::home_dir;


use serde;
use serde_derive;
use serde::ser::Serialize;
use serde::de::Deserialize;
use serde::de::Error;

use toml;

mod background;
use self::background::BackgroundConfig;

mod keyboard;
use self::keyboard::KeyboardConfig;

mod layout;
use self::layout::LayoutConfig;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub background: BackgroundConfig,
    pub keyboard: KeyboardConfig,
    pub layout: LayoutConfig
}

impl Default for Config {
    fn default() -> Self {
        Config {
            background: BackgroundConfig::default(),
            keyboard: KeyboardConfig::default(),
            layout: LayoutConfig::default()
        }
    }
}

impl Config{
    pub fn file_path() -> PathBuf {
        let mut config_path = home_dir().unwrap();

        config_path.push(".config");
        config_path.push("rustland");
        config_path.push("config.toml");
        
        return config_path;
    }

    pub fn load_from_file(toml_config_file: PathBuf) -> Option<Config>{
        let file = match toml_config_file.exists() {
            true => File::open(toml_config_file),
            false => {
                println!("No configuration file found - creating a new one at: {}", toml_config_file.to_str().unwrap());
                Config::default().save_to_file(toml_config_file.clone());
                File::open(toml_config_file)
            }          
        };
        
        match file {
            Ok(mut valid_config_file) => {
                let mut contents = String::new();
                valid_config_file.read_to_string(&mut contents);
                
                match toml::from_str(&contents) {
                    Ok(text) => Some(text),
                    Err(_) => None
                }
            },
            Err(_) => {
                println!("Couldn't open file for that!");
                None
            }
        } 
    }

    pub fn save_to_file(&self, toml_config_file: PathBuf){
        let mut parent_directory = toml_config_file.clone();
        parent_directory.pop();        
        create_dir_all(parent_directory);

        match File::create(toml_config_file) {
            Ok(mut file) => {
                match toml::to_string(self) {
                    Ok(data) => { file.write_all(data.as_bytes()); },
                    Err(e) => { println!("Couldn't overwrite config: {}", e); }
                };
            },
            Err(e) => {
                println!("Couldn't create config file: {}", e);
            }
        }
    }
}