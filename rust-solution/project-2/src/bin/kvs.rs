use clap::{App, Arg, SubCommand};
use std::process;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");
const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

fn do_set(k: String, v: String) -> kvs::Result<()> {
    let mut store = kvs::KvStore::open(std::env::current_dir()?.as_path())?;
    store.set(k, v)
}

fn do_get(k: String) -> kvs::Result<Option<String>> {
    let store = kvs::KvStore::open(std::env::current_dir()?.as_path())?;
    store.get(k)
}

fn do_rm(k: String) -> kvs::Result<Option<String>> {
    let mut store = kvs::KvStore::open(std::env::current_dir()?.as_path())?;
    store.remove(k)
}

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

    if let Some(set) = m.subcommand_matches("set") {
        match do_set(set.value_of("KEY").unwrap().to_owned(), set.value_of("VALUE").unwrap().to_owned()) {
            Err(_) => process::exit(1),
            Ok(_) => process::exit(0),
        }
    }
    if let Some(get) = m.subcommand_matches("get") {
        match do_get(get.value_of("KEY").unwrap().to_owned()) {
            Err(_) => process::exit(1),
            Ok(m) => {
                match m {
                    Some(v) => {
                        println!("{}", v);
                    },
                    None => {
                        println!("Key not found");
                        process::exit(0);
                    }

                };
                process::exit(0);
            }
        }
    }
    if let Some(rm) = m.subcommand_matches("rm") {
        match do_rm(rm.value_of("KEY").unwrap().to_owned()) {
            Err(e) => {
                match e.downcast::<kvs::KeyNotFoundOnRemove>() {
                    Ok(_) => {
                        println!("Key not found");
                        process::exit(1);
                    }
                    Err(_) => {
                        process::exit(1);
                    }
                }
            }
            Ok(m) => {
                match m {
                    Some(_) => {},
                    None => {
                        println!("Key not found");
                        process::exit(1);
                    }
                };
                process::exit(0);
            }
        }
    }
    // no args?
    process::exit(1);
}
