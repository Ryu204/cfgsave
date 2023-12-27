use std::env;
use std::path::PathBuf;
use std::str::FromStr;

pub fn home_dir() -> Result<PathBuf, String> {
    let home_string = match env::var("HOME") {
        Ok(home) => home,
        Err(err) => return Err(err.to_string())
    };
    match PathBuf::from_str(&home_string) {
        Ok(path) => Ok(path),
        Err(err) => Err(err.to_string())
    }
}

pub fn root() -> Result<PathBuf, String> {
    match PathBuf::from_str("/") {
        Ok(path) => Ok(path),
        Err(err) => Err(err.to_string())
    }
}