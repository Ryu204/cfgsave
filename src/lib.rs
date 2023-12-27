pub mod runner;
pub use runner::cmd;
pub mod core;

use cmd::Command;

const HELP_MESSAGE: &str = 
r"Usage:
    list            List all file(s) currently tracked.
    add <name>      Add a file to be tracked.
    remove <name>   Remove a file from tracked list.
    snap            Update live status of tracked file(s).
    publish [quiet] Publish tracked file(s) to original address.
Have a good day!";

pub fn run(command: Command) -> Result<(), String> {
    match command {
        Command::List => runner::list::execute(),
        Command::Add(filename) => runner::add::execute(&filename),
        Command::Remove(filename) => runner::remove::execute(&filename),
        Command::Snap => runner::snap::execute(),
        Command::Publish(quiet) => runner::publish::execute(quiet.quiet_yes),
        Command::None => {
            println!("{}", HELP_MESSAGE);
            Ok(())
        }
        Command::Err(err) => Err(err.to_string())
    }
}