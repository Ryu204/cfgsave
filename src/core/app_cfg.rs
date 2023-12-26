use std::env;
use std::fs;
use std::path::PathBuf;

pub static APP_NAME : &str = "cfgsave";

pub enum FileType {
    Data
}
/// Defines path to needed files. 
/// 
/// Eg. data, config, .etc
pub struct AppInfo  {
}

impl AppInfo {
    pub fn get_path(file_type: FileType) -> Result<PathBuf, String> {
        match file_type {
            FileType::Data => Self::build_pathbuf("data.txt")
        }
    }

    pub fn create_if_not_exist(file_type: FileType) -> Result<PathBuf, String> {
        let filepath = match Self::get_path(file_type) {
            Ok(path) => path,
            Err(err) => return Err(err)
        };
        let file_exists = fs::metadata(&filepath).is_ok();
        if file_exists {
            return Ok(filepath);
        }
        let parent_dir = match filepath.parent() {
            Some(path) => path,
            None => return Err(format!("Cannot locate directory of {:?}", filepath.as_os_str()))
        };
        if let Err(err) = fs::create_dir_all(parent_dir) {
            return Err(err.to_string())
        }
    
        match fs::write(&filepath, "") {
            Ok(_) => Ok(filepath),
            Err(err) => Err(format!("Cannot create file {:?}. Details:\n{}", filepath, err.to_string())),
        }
    }

    pub fn storage_dir() -> Result<PathBuf, String> {
        match env::var("HOME") {
            Ok(home_dir) => Ok([home_dir + "/", ".".to_owned() + APP_NAME].iter().collect()),
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