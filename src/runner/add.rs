use crate::core::data;

pub fn execute(filename: &str) -> Result<(), String> {
    let file_to_add = match check_existence(filename) {
        Ok(file) => file,
        Err(err) => return Err(err)
    };
    let mut data = match data::Data::open() {
        Ok(data) => data,
        Err(err) => return Err(err)
    };
    data.insert(file_to_add);
    Ok(())
}

fn check_existence(filename: &str) -> Result<data::File, String> {
    let file = match data::File::from(filename) {
        Ok(file) => file,
        Err(err) => return Err(err)
    };
    match file.exists() {
        Ok(status) => if status { Ok(file) } else { Err(format!("{:?} does not exist", filename)) }
        Err(err) => Err(err)
    }
}