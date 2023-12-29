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

pub fn cwd() -> Result<PathBuf, String> {
    match std::env::current_dir() {
        Ok(dir) => Ok(dir),
        Err(err) => Err(err.to_string())
    }
}

/// Convert relative address (to current working directory) to absolute.
/// 
/// If `path` is absolute, the result is `Ok(path)`
pub fn absolute_path_by_cwd(path: &str) -> Result<String, String> {
    match cwd() {
        Ok(dir) => Ok(dir.join(path).to_string_lossy().to_string()),
        Err(err) => Err(format!("Cannot locate current working directory: {}", err))
    }
}
