pub struct Publish {
    pub quiet_yes: bool,
}

pub enum Command {
    None,
    List,
    Add(String),
    Remove(String),
    Snap,
    Publish(Publish),
    Err(String),
}

pub fn parse(args: &Vec<String>) -> Command {
    if args.len() == 1 {
        Command::None
    }
    else if args[1] == "list" {
        if args.len() > 2 {
            return Command::Err(String::from("\"list\" does not take parameters."));
        }
        Command::List
    }
    else if args[1] == "add" {
        if args.len() != 3 {
            return  Command::Err(String::from("Usage: add <absolute_filename>."));
        }
        Command::Add(args[2].clone())
    }
    else if args[1] == "remove" {
        if args.len() != 3 {
            return  Command::Err(String::from("Usage: remove <absolute_filename>."));
        }
        Command::Remove(args[2].clone())
    }
    else if args[1] == "snap" {
        if args.len() > 2 {
            return Command::Err(String::from("\"snap\" does not take parameters."));
        }
        Command::Snap
    }
    else if args[1] == "publish" {
        if args.len() == 2 {
            Command::Publish(Publish{quiet_yes: false})
        }
        else {
            if args.len() != 3 || args[2] != "quiet" {
                Command::Err(String::from("Usage: publish [quiet]."))
            }
            else {
                Command::Publish(Publish{quiet_yes: true})
            }
        }
    }
    else {
        Command::Err(String::from("Unknown command."))
    }
}