extern crate clap;

use clap::{App, Arg, SubCommand};
use kvs::KvStore;

fn main() {
    let matches = App::new("kvs")
        .arg(Arg::with_name("version").short("V").long("version"))
        .subcommands(vec![
            SubCommand::with_name("get")
                .about("get the value of key")
                .author(env!("CARGO_PKG_AUTHORS"))
                .version(env!("CARGO_PKG_VERSION"))
                .arg(Arg::with_name("key").index(1).required(true)),
            SubCommand::with_name("rm")
                .about("rm the key")
                .author(env!("CARGO_PKG_AUTHORS"))
                .version(env!("CARGO_PKG_VERSION"))
                .arg(Arg::with_name("key").index(1).required(true)),
            SubCommand::with_name("set")
                .about("set a key-value pair")
                .author(env!("CARGO_PKG_AUTHORS"))
                .version(env!("CARGO_PKG_VERSION"))
                .args(&[
                    Arg::with_name("key").index(1).required(true),
                    Arg::with_name("value").index(2).required(true),
                ]),
        ])
        .get_matches();

    if matches.is_present("version") {
        println!(env!("CARGO_PKG_VERSION"));
        return;
    }

    let mut kv = KvStore::new();
    match matches.subcommand_name() {
        Some("get") => {
            if let Some(get_matches) = matches.subcommand_matches("get") {
                let key = String::from(get_matches.value_of("key").unwrap());
                kv.get(key);
            }
            panic!("unimplemented");
        }
        Some("rm") => {
            if let Some(rm_matches) = matches.subcommand_matches("rm") {
                let key = String::from(rm_matches.value_of("key").unwrap());
                kv.remove(key);
            }
            panic!("unimplemented");
        }
        Some("set") => {
            if let Some(set_matches) = matches.subcommand_matches("set") {
                let key = String::from(set_matches.value_of("key").unwrap());
                let value = String::from(set_matches.value_of("value").unwrap());
                kv.set(key, value);
            }
            panic!("unimplemented");
        }
        None => {
            panic!("unimplemented");
        }
        _ => unreachable!(),
    }
}
