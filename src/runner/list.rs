use crate::core::data;

pub fn execute() -> Result<(), String> {
    let data = match data::Data::open() {
        Ok(data) => data,
        Err(err) => return Err(err)
    };
    if data.len() == 0 {
        println!("No file tracked.");
        return Ok(());
    }
    println!("{} file(s) tracked:\n{}", data.len(), data.list());
    Ok(())
}