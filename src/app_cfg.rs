use std::env;
use std::path::PathBuf;

pub static APP_NAME : &str = "cfgsave";

/// Defines path to needed files. 
/// 
/// Eg. data, config, .etc
pub struct Files {
    data: PathBuf,
}

impl Files {
    pub fn new() -> Result<Self, String> {
        let mut maybe_res = Self {
            data: PathBuf::new()
        };
        match Self::get_data() {
            Ok(path) => maybe_res.data = path,
            Err(err) => return Err(err.to_string())
        }
        Ok(maybe_res)
    }

    pub fn data(&self) -> &PathBuf {
        &self.data
    }

    fn get_data() -> Result<PathBuf, String> {
        Self::build_pathbuf("data.ini")
    }

    fn storage_dir() -> Result<PathBuf, String> {
        match env::var("HOME") {
            Ok(home_dir) => Ok([home_dir + "/", ".config/".to_owned(), APP_NAME.to_owned()].iter().collect()),
            Err(err) => Err(err.to_string())
        }
    }

    fn build_pathbuf(name: &str) -> Result<PathBuf, String> {
        match Self::storage_dir() {
            Err(err) => Err(format!("Cannot find home directory. Details:\n{}", err)),
            Ok(path) => Ok(path.join(name))
        }
    }
}