use crate::core;

pub fn execute(filename: &str) -> Result<(), String> {
    let file_to_add = match check_existence(filename) {
        Ok(file) => file,
        Err(err) => return Err(err)
    };
    let mut data = match core::Data::open() {
        Ok(data) => data,
        Err(err) => return Err(err)
    };
    match file_to_add.update() {
        core::FileUpdate::Err(err) => return Err(err),
        core::FileUpdate::Removed => return Err(String::from("File has been removed.")),
        _ => ()
    };    
    println!("Adding {:?} to tracked list.", file_to_add.filename());
    data.insert(file_to_add);
    data.save()
}

fn check_existence(filename: &str) -> Result<core::File, String> {
    let file = match core::File::from(filename) {
        Ok(file) => file,
        Err(err) => return Err(err)
    };
    match file.exists() {
        Ok(status) => if status { Ok(file) } else { Err(format!("{:?} does not exist", filename)) }
        Err(err) => Err(err)
    }
}