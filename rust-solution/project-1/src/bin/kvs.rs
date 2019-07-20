use clap::{App, Arg, SubCommand};
use std::process;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");
const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

fn main() {
    let a = App::new(NAME.unwrap_or("kvs"))
        .version(VERSION.unwrap_or("UNKNOWN"))
        .author(AUTHORS.unwrap_or("UNKNOWN"))
        .about(DESCRIPTION.unwrap_or("UNKNOWN"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").required(true).index(1))
                .arg(Arg::with_name("VALUE").required(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").required(true).index(1)),
        );
    let m = a.get_matches();

    if let Some(_set) = m.subcommand_matches("set") {
        eprintln!("unimplemented");
        process::exit(1);
    }
    if let Some(_get) = m.subcommand_matches("get") {
        eprintln!("unimplemented");
        process::exit(1);
    }
    if let Some(_rm) = m.subcommand_matches("rm") {
        eprintln!("unimplemented");
        process::exit(1);
    }
    // no args?
    process::exit(1);
}
