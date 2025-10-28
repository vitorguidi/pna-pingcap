use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("kvs")
        .about("kvs for rust")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("set")
                .short_flag('S')
                .long_flag("set")
                .about("Set a key/value pair.")
                .arg(
                    Arg::new("key")
                        .action(ArgAction::Set)
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("value")
                        .require_equals(true)
                        .index(2)
                ),
        )
        .subcommand(
            Command::new("get")
                .short_flag('G')
                .long_flag("get")
                .about("Get a value from a key.")
                .arg(
                    Arg::new("key")
                        .action(ArgAction::Set)
                        .required(true)
                        .index(1),
                )
        )
        .subcommand(
            Command::new("rm")
                .short_flag('R')
                .long_flag("rm")
                .about("Remove a key/value pair.")
                .arg(
                    Arg::new("key")
                        .action(ArgAction::Set)
                        .required(true)
                        .index(1),
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", matches)) => {
            dbg!(matches);
            let key: Option<&String> = matches.get_one("key");
            let value: Option<&String> = matches.get_one("value");
            println!("Setting key: {}, value: {}", key.unwrap(), value.unwrap());
        },
        Some(("get", matches)) => {
            dbg!(matches);
            let key: Option<&String> = matches.get_one("key");
            println!("Getting key: {}", key.unwrap());
        },
        Some(("rm", matches)) => {
            dbg!(matches);
            let key: Option<&String> = matches.get_one("key");
            println!("Removing key: {}", key.unwrap());            
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
