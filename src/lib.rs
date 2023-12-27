pub mod runner;
pub use runner::cmd;
pub mod core;

use cmd::Command;

pub fn run(command: Command) -> Result<(), String> {
    match command {
        Command::List => runner::list::execute(),
        Command::Add(filename) => runner::add::execute(&filename),
        Command::Remove(filename) => runner::remove::execute(&filename),
        Command::None => {
            println!("Have a good day!");
            Ok(())
        }
        Command::Err(err) => Err(err.to_string())
    }
}