use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let cmd = cfgsave::cmd::parse(&args);
    match cfgsave::run(&cmd) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("An error occurred.\n{}", err);
            ExitCode::FAILURE
        }
    }
}
