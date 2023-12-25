pub mod runner;
pub use runner::cmd;
pub mod app_cfg;

pub fn run(command: &Option<cmd::Command>) -> Result<(), String> {
    match command {
        None => {
            println!("Have a nice day!");
            Ok(())
        }
        Some(cmd) => match cmd {
            cmd::Command::List => runner::list::execute(),
        }
    }
}