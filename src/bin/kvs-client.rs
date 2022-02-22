use std::{net::SocketAddr, process::exit};

use clap::{Parser, Subcommand};

use kvs::{KvsClient, Result};

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const ADDRESS_FORMAT: &str = "IP:PORT";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(name = "kvs-client")]
#[clap(disable_help_subcommand(true))]
#[clap(subcommand_required(true))]
#[clap(arg_required_else_help(true))]
struct Opt {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(name = "get", about = "Get the string value of a given string key")]
    Get {
        #[clap(name = "KEY", help = "A string key")]
        key: String,
        #[clap(
            long,
            help = "Sets the server address",
            value_name = ADDRESS_FORMAT,
            default_value = DEFAULT_LISTENING_ADDRESS,
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
    #[clap(name = "set", about = "Set the value of a string key to a string")]
    Set {
        #[clap(name = "KEY", help = "A string key")]
        key: String,
        #[clap(name = "VALUE", help = "The string value of the key")]
        value: String,
        #[clap(
            long,
            help = "Sets the server address",
            value_name = ADDRESS_FORMAT,
            default_value = DEFAULT_LISTENING_ADDRESS,
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
    #[clap(name = "rm", about = "Remove a given string key")]
    Remove {
        #[clap(name = "KEY", help = "A string key")]
        key: String,
        #[clap(
            long,
            help = "Sets the server address",
            value_name = ADDRESS_FORMAT,
            default_value = DEFAULT_LISTENING_ADDRESS,
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
}

fn main() {
    let opt = Opt::parse();
    if let Err(e) = run(opt) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    match opt.command {
        Some(Command::Get { key, addr }) => {
            let mut client = KvsClient::connect(addr)?;
            if let Some(value) = client.get(key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Some(Command::Set { key, value, addr }) => {
            let mut client = KvsClient::connect(addr)?;
            client.set(key, value)?;
        }
        Some(Command::Remove { key, addr }) => {
            let mut client = KvsClient::connect(addr)?;
            client.remove(key)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}
