pub mod runner;
pub use runner::cmd;
pub mod core;

pub fn run(command: cmd::Command) -> Result<(), String> {
    match command {
        cmd::Command::List => runner::list::execute(),
        cmd::Command::Add(filename) => runner::add::execute(&filename),
        cmd::Command::None => {
            println!("Have a good day!");
            Ok(())
        }
        cmd::Command::Err(err) => Err(err.to_string())
    }
}