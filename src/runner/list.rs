use std::{fs, path::PathBuf};
use crate::app_cfg;

pub fn execute() -> Result<(), String> {
    let files = match app_cfg::Files::new() {
        Ok(path) => path,
        Err(err) => return Err(err)
    };
    let filepath = files.data();
    if let Err(err) = create_if_not_exist(filepath) {
        return Err(format!("Cannot create data file in {:?}. Details:\n{}", 
            files.data().as_os_str(), err));
    }
    let content = match fs::read_to_string(&files.data()) {
        Err(err) => return Err(format!("Cannot read data file in {:?}. Details:\n{}", 
            filepath.as_os_str(), err.to_string())),
        Ok(content) => content
    };
    match process_content(&content) {
        Err(err) => Err(format!("Data file is corrupted. Details:\n{:?}:\n{}", 
            filepath.as_os_str(), err.to_string())),
        Ok(_) => Ok(())
    }
}

fn create_if_not_exist(filepath: &PathBuf) -> Result<(), String> {
    let file_exists = fs::metadata(filepath).is_ok();
    if file_exists {
        return Ok(());
    }
    let parent_dir = match filepath.parent() {
        Some(path) => path,
        None => return Err(format!("Cannot locate directory of {:?}", filepath.as_os_str()))
    };
    if let Err(err) =  fs::create_dir_all(parent_dir) {
        return Err(err.to_string())
    }

    match fs::write(filepath, "") {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

fn process_content(content: &str) -> Result<(), String> {
    if content != "he" {
        return Ok(());
    }
    else { 
        return Ok(());
    }
}