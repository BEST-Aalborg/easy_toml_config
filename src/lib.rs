extern crate serde;
pub extern crate toml;
#[macro_use]
extern crate lazy_static;

use std::env::home_dir;
use std::fs::{create_dir_all,File};
#[allow(unused_imports)]
use std::io::{Read,Write};
use std::path::{PathBuf};
use std::sync::Mutex;

use self::serde::Deserialize;
use self::serde::Serialize;

lazy_static! {
    static ref CONFIG_FOLDER: Mutex<Option<String>> = Mutex::new(None);
}

pub fn set_config_dir(path: String) -> bool {
    let mut conf = CONFIG_FOLDER.lock().unwrap();
    if conf.is_none() {
        *conf = Some(path);
        return true;
    } else {
        return false;
    }
}

pub fn get_config_dir() -> String {
    let conf = CONFIG_FOLDER.lock().unwrap();
    if conf.is_none() {
        panic!("The function set_config_dir, must be run ones before you can the function get_config_dir is called");
    } else {
        return conf.clone().unwrap();
    }
}


pub fn init<'de, T>(filename: &str, config: T) -> File where T: Deserialize<'de> + Serialize + WriteConfig {
    let path_config_file = path_config_file(filename);
    let config_file: File;

    let path_config_dir = path_config_dir();
    create_dir_all(path_config_dir.as_path()).expect(&format!("Failed to create the directory '{}'", path_config_file.to_str().unwrap()));

    if path_config_file.exists() {
        if path_config_file.is_file() {
            config_file = File::open(&path_config_file).expect(&format!("unable to open the config file '{}'", path_config_file.to_str().unwrap()));
        } else {
            panic!("Cannot access the config file '{}'", path_config_file.to_str().unwrap());
        }
    } else {
        config.write();
        ::std::process::exit(1);
    }

    config_file
}

pub fn error_handler<T>(config: serde::export::Result<T, toml::de::Error>) -> T where T: self::serde::export::fmt::Debug {
    if config.is_err() {
        let error = &config.unwrap_err().inner;
        match error.kind {
            toml::de::ErrorKind::Custom => {
                println!("You have to add the {}, in the section [{}] of the config file", error.message, error.key.first().unwrap());
            }
            _ => {
                println!("-=( Un-handled error )=-");
                println!("{:?}", error);
            }
        }

        ::std::process::exit(1);
    }

    config.unwrap()
}

fn path_config_dir() -> PathBuf {
    let mut path_config_dir = home_dir().unwrap();
    path_config_dir.push(get_config_dir());
    path_config_dir
}

fn path_config_file(filename: &str) -> PathBuf {
    let mut path_config_file = path_config_dir();
    path_config_file.push(filename);
    path_config_file
}

pub trait WriteConfig {
    fn write(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}
