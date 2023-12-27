use std::{path::PathBuf, io::Write};
use std::str::FromStr;
use std::fs;
use crate::core::{AppInfo, FileType};

pub struct File {
    path: PathBuf,
}

pub enum FileUpdate {
    Exist,
    Removed,
    Err(String)
}

pub enum FilePublish {
    Ok,
    Abort,
    Err(String)
}

impl File {
    pub fn from(filename: &str) -> Result<Self, String> {
        let mut maybe_res = Self {
            path: PathBuf::new(),
        };
        maybe_res.path = match PathBuf::from_str(filename) {
            Ok(path) => {
                if !path.is_absolute() {
                    return Err(format!("{:?} is not an absolute path", path))
                }
                path
            }
            Err(err) => return Err(err.to_string())
        };
        Ok(maybe_res)
    }
    pub fn filename(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
    pub fn exists(&self) -> Result<bool, String> {
        match self.path.try_exists() {
            Err(err) => return Err(err.to_string()),
            Ok(state) => if state == false { return Ok(false) }
        };
        if self.path.is_file() { Ok(true) }
        else {Err(format!("{:?} is a folder", self.path))}
    }
    pub fn update(&self) -> FileUpdate {
        let write_path = match self.get_write_path() {
            Ok(path) => path,
            Err(err) => return FileUpdate::Err(err)
        };
        match self.exists() {
            Ok(state) => if state == false {
                return match Self::remove_file(&write_path) {
                    Ok(_) => FileUpdate::Removed,
                    Err(err) => FileUpdate::Err(err)
                }
            }
            Err(err) => return FileUpdate::Err(err)
        };
        let content = match std::fs::read_to_string(&self.path) {
            Ok(content) => content,
            Err(err) => return FileUpdate::Err(
                format!("Cannot read {:?}. Details:\n{}", 
                self.path, err.to_string())
            )
        };
        match Self::write_file(&write_path, &content) {
            Ok(_) => FileUpdate::Exist,
            Err(err) => FileUpdate::Err(format!(
                "Cannot write {:?}. Details:\n{}",
                write_path, err.to_string()
            ))
        }
    }
    pub fn publish(&self, quiet: bool) -> FilePublish {
        let existed = match self.exists() {
            Err(err) => return FilePublish::Err(err),
            Ok(state) => state
        };
        if !quiet {
            print!("{}? [y(es),n(o)] ", if existed { "Overwrite" } else { "Write" });
            std::io::stdout().flush().unwrap();
            let mut response = String::new(); 
            std::io::stdin().read_line(&mut response).unwrap();
            let response = match response.trim().to_lowercase().as_str() {
                "yes" => true,
                "y" => true,
                "no" => false,
                "n" => false,
                _ => return FilePublish::Err(String::from("Invalid response."))
            };
            if response == false {
                return FilePublish::Abort;
            }
        }
        let write_path = match self.get_write_path() {
            Ok(path) => path,
            Err(err) => return FilePublish::Err(err)
        };
        let content = match std::fs::read_to_string(&write_path) {
            Ok(content) => content,
            Err(err) => return FilePublish::Err(format!(
                "Cannot read {:?}: {}", write_path, err.to_string()))
        };
        match Self::write_file(&self.path, &content) {
            Ok(_) => FilePublish::Ok,
            Err(err) => FilePublish::Err(format!("Cannot write {:?}: {}", &self.path, err))
        }
    }
    fn remove_file(path: &PathBuf) -> Result<(), String> {
        match path.try_exists() {
            Ok(state) => if state == false {return Ok(())}
            Err(err) => return Err(format!("Cannot access {:?}: {}", path, err.to_string()))
        };
        match fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())            
        }
    }
    fn get_write_path(&self) -> Result<PathBuf, String> {
        let base = match AppInfo::get_path(FileType::Base) {
            Ok(path) => path,
            Err(err) => return Err(err)
        }; 
        let relative_to_root = match AppInfo::strip_root(&self.path) {
            Ok(path) => path,
            Err(err) => return Err(err)
        };
        Ok(base.join(relative_to_root))
    }
    fn touch_file(path: &PathBuf) -> Result<(), String> {
        let dir = match path.parent() {
            Some(parent) => parent,
            None => return Err(String::from("Cannot locate parent dir"))
        };
        match fs::create_dir_all(dir) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Cannot create {:?}: {}", dir, err.to_string()))
        }
    }
    fn write_file(path: &PathBuf, content: &str) -> Result<(), String> {
        if let Err(err) = Self::touch_file(&path) {
            return Err(err)
        }
        match std::fs::write(&path, content) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        }
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        return self.path.eq(&other.path);
    }
}
