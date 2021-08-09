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
        Some(Command::Get { key }) => {
            kv.get(key);
            panic!("unimplemented");
        }
        Some(Command::Rm { key }) => {
            kv.remove(key);
            panic!("unimplemented");
        }
        Some(Command::Set { key, value }) => {
            kv.set(key, value);
            panic!("unimplemented");
        }
        None => {
            panic!("unimplemented");
        }
    }
}
