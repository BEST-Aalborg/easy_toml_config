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

pub enum Error {
    DirNeverSpecified
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

pub fn set_config_dir_as_path(path: PathBuf) -> bool {
    set_config_dir(path.to_str().unwrap().to_string())
}

pub fn get_config_dir() -> Result<String, Error> {
    let conf = CONFIG_FOLDER.lock().unwrap();
    if conf.is_none() {
        Err(Error::DirNeverSpecified)
    } else {
        return Ok(conf.clone().unwrap());
    }
}


pub fn init<'de, T>(filename: &str, config: &T) -> Result<File, Error> where T: Deserialize<'de> + Serialize + WriteConfig {
    let path_config_file = match path_config_file(filename) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let config_file: File;

    let path_config_dir = path_config_dir();
    match path_config_dir {
        Ok(path) => create_dir_all(path.as_path()).expect(&format!("Failed to create the directory '{}'", path_config_file.to_str().unwrap())),
        Err(e) => return Err(e),
    }

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

    Ok(config_file)
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

pub fn path_config_dir() -> Result<PathBuf, Error> {
    get_config_dir().and_then(|dir| {
        let mut path_config_dir = home_dir().unwrap();
        path_config_dir.push(dir);
        Ok(path_config_dir)
    })
}

pub fn path_config_file(filename: &str) -> Result<PathBuf, Error> {
    path_config_dir().and_then(|path| {
        let mut path_config_file = path;
        path_config_file.push(filename);
        Ok(path_config_file)
    })
}

pub trait WriteConfig {
    fn write(&self);
}