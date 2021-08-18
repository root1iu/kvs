extern crate structopt;

use kvs::KvStore;
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

    let mut kv = KvStore::new();
    match opt.cmd {
        Some(Command::Get { key }) => match kv.get(key) {
            Ok(value) => println!("get value: {}", value),
            Err(err) => panic!("get fail for {}", err),
        },
        Some(Command::Rm { key }) => match kv.remove(key) {
            Ok(value) => println!("remove value: {}", value),
            Err(err) => panic!("remove fail for {}", err),
        },
        Some(Command::Set { key, value }) => match kv.set(key, value) {
            Ok(value) => println!("set old value: \"{}\"", value),
            Err(err) => panic!("set fail for {}", err),
        },
        None => {
            panic!("unimplemented");
        }
    }
}
