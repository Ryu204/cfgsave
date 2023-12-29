use crate::core;

pub fn execute() -> Result<(), String> {
    let mut data = match core::Data::open() {
        Ok(data) => data,
        Err(err) => return Err(err)
    };
    if data.len() == 0 {
        println!("No file tracked.");
        Ok(())
    }
    else {
        match data.snap() {
        Err(err) => Err(format!("Fatal error(s)!\n{}", err)),
        Ok(_) => Ok(())
        }
    }
}