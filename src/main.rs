use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// File used for key/value storage
    #[structopt(short, long, env = "KV_PATH", default_value = "./var/sled.db")]
    kv_path: PathBuf,

    /// SQLite database
    #[structopt(short, long, env = "SQLITE_DB_PATH", default_value = "./var/data.db")]
    sqlite_path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
