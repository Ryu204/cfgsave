pub enum Command {
    List
}

pub fn parse(args: &Vec<String>) -> Option<Command> {
    if args.len() == 1 {
        None
    }
    else if args[1] == "list" {
        Some(Command::List)
    }
    else {
        None
    }
}