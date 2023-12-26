use std::path::PathBuf;
use std::str::FromStr;
use std::fs;
use crate::core::app_cfg::AppInfo;
use crate::core::app_cfg::FileType;

pub struct File {
    path: PathBuf,
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
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        return self.path.eq(&other.path);
    }
}

pub struct Data {
    files: Vec<File>,
}

impl Data {
    pub fn open() -> Result<Self, String> {
        let data_path = match AppInfo::create_if_not_exist(FileType::Data) {
            Ok(path) => path,
            Err(err) => return Err(err)
        };
        let content = match fs::read_to_string(&data_path) {
            Err(err) => return Err(format!("Cannot read file {:?}. Details:\n{}",data_path, err)),
            Ok(content) => content
        };
        match Self::from(&content) {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("Data file is corrupted. Details:\n{:?}\n{}", data_path, err))
        }
    }
    pub fn list(&self) -> String {
        let mut res = String::new();
        for file in &self.files {
            res.push_str(&file.filename());
            res.push_str("\n");
        }
        res
    }
    pub fn contains(&self, file: &File) -> bool {
        return self.files.contains(file);
    }
    pub fn insert(&mut self, file: File) {
        if self.contains(&file) { return; }
        self.files.push(file);
    }
    pub fn len(&self) -> usize {
        self.files.len()
    }
    fn from(content: &str) -> Result<Self, String> {
        let names: Vec<&str> = content.split_whitespace().collect();
        let mut files: Vec<File> = Vec::new();
        for name in &names {
            match File::from(name) {
                Ok(file) => files.push(file),
                Err(err) => return Err(err)
            }
        }
        Ok(Self { files })
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        let path = match AppInfo::get_path(FileType::Data) {
            Ok(path) => path,
            Err(_) => return
        };
        let _ = fs::write(path, self.list());
    }
}
