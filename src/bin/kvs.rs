use std::{env::current_dir, process::exit};

use clap::{Arg, Command};
use kvs::{KvStore, KvsEngine, KvsError, Result};

fn main() -> Result<()> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .disable_help_subcommand(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::new("KEY").help("A string key").required(true))
                .arg(
                    Arg::new("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a given key")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let value = matches.value_of("VALUE").unwrap();

            let store = KvStore::open(current_dir()?)?;
            store.set(key.to_string(), value.to_string())?;
        }
        Some(("get", matches)) => {
            let key = matches.value_of("KEY").unwrap();

            let store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Some(("rm", matches)) => {
            let key = matches.value_of("KEY").unwrap();

            let store = KvStore::open(current_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(()) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
