extern crate structopt;

use kvs::KvStore;
use std::env;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "kvs", about = "an in-memory key/value store")]
struct Opt {
    #[structopt(name = "version", long = "version", short = "V")]
    version: bool,
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "get")]
    Get { key: String },
    #[structopt(name = "rm")]
    Rm { key: String },
    #[structopt(name = "set")]
    Set { key: String, value: String },
}

fn main() {
    let opt = Opt::from_args();

    if opt.version {
        println!(env!("CARGO_PKG_VERSION"));
        return;
    }

    let cwd = env::current_dir().unwrap();
    let mut kv = KvStore::open(cwd).unwrap();
    match opt.cmd {
        Some(Command::Get { key }) => match kv.get(key) {
            Ok(value) => match value {
                Some(v) => println!("{}", v),
                None => println!("Key not found"),
            },
            Err(_) => {}
        },
        Some(Command::Rm { key }) => match kv.remove(key) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        },
        Some(Command::Set { key, value }) => match kv.set(key, value) {
            Err(err) => panic!("set fail for {}", err),
            _ => (),
        },
        None => {
            panic!("unimplemented");
        }
    }
}
