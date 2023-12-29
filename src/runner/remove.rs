use crate::core;

pub fn execute(filename: &str) -> Result<(), String> {
    let filename = match core::absolute_path_by_cwd(filename) {
        Ok(new_name) => new_name,
        Err(err) => return Err(err)
    };
    let mut data = match core::Data::open() {
        Ok(data) => data,
        Err(err) => return Err(err)
    };
    let file = match core::File::from(&filename) {
        Ok(file) => file,
        Err(err) => return Err(err)
    };
    if !data.contains(&file) {
        return Err(format!("{:?} is not tracked.", file.filename()))
    }
    println!("Removing {:?} from tracked list.", file.filename());
    data.remove(&file);
    data.save()
}