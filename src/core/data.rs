use std::fs;
use crate::core::file::File;
use crate::core::file::FileUpdate;
use crate::core::app_cfg::AppInfo;
use crate::core::app_cfg::FileType;
use crate::core::file::FilePublish;

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
    pub fn remove(&mut self, file: &File) {
        self.files.retain(|x| !x.eq(file));
    }
    pub fn len(&self) -> usize {
        self.files.len()
    }
    pub fn save(&self) -> Result<(), String> {
        let path = match AppInfo::get_path(FileType::Data) {
            Ok(path) => path,
            Err(err) => return Err(err)
        };
        match fs::write(&path, self.list()) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Cannot save {:?}. Details:\n{}", path, err.to_string()))
        }
    }
    pub fn snap(&mut self) -> Result<(), String> {
        let mut error_log = String::new();
        self.files.retain(|x| {
            match x.update() {
                FileUpdate::Err(err) => {
                    error_log += &err;
                    error_log += "\n";
                    true
                }
                FileUpdate::Exist => {
                    println!("+\tSnapped {:?}.", x.filename());
                    true
                }
                FileUpdate::Removed => {
                    println!("-\tRemoved {:?}.", x.filename());
                    false
                }
            }
        });
        if !error_log.is_empty() {
            Err(error_log)
        }
        else {
            Ok(())
        }
    }
    pub fn publish(&self, quiet_yes: bool) -> Result<(), String> {
        println!("Number of file(s) tracked: {}", self.len());
        let mut error_log = String::new();
        let mut count = 0;
        let mut error_count = 0;
        for file in &self.files {
            count += 1;
            println!("#{}. {:?}", count, file.filename());
            if let FilePublish::Err(err) = file.publish(quiet_yes) {
                error_log = format!("{}#{}. {}", 
                if error_log.is_empty() {String::new()} else {error_log + "\n"}
                , count, err.as_str());
                error_count += 1;
                println!("(!) FAILED");
            }
            else {
                println!("(^) OK")
            }
        }
        if !error_log.is_empty() {
            Err(format!("{} file(s) failed:\n{}", error_count, error_log))
        }
        else {
            Ok(())
        }
    }
    fn from(content: &str) -> Result<Self, String> {
        let names: Vec<&str> = content.split("\n").filter(|x| !x.is_empty()).collect();
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