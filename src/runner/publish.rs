use crate::core;

pub fn execute(quiet: bool) -> Result<(), String> {
    let data = match core::Data::open() {
        Ok(data) => data,
        Err(err) => return Err(err)
    };
    match data.publish(quiet) {
        Ok(_) => {
            println!("Publish done.");
            Ok(())
        }
        Err(err) => Err(err)
    }
}