use crate::core::data;

pub fn execute(filename: &str) -> Result<(), String> {
    let mut data = match data::Data::open() {
        Ok(data) => data,
        Err(err) => return Err(err)
    };
    let file = match data::File::from(filename) {
        Ok(file) => file,
        Err(err) => return Err(err)
    };
    if !data.contains(&file) {
        Err(format!("{:?} is not tracked.", file.filename()))
    }
    else {
        data.remove(&file);
        data.save()
    }
}