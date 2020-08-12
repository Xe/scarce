use anyhow::Result;
use rusqlite::params;
use rusqlite::Connection as SQLiteDB;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// File used for key/value storage
    #[structopt(short, long, env = "KV_PATH", default_value = "./var/sled")]
    kv_path: PathBuf,

    /// SQLite database
    #[structopt(short, long, env = "SQLITE_DB_PATH", default_value = "./var/data.db")]
    sqlite_path: PathBuf,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let sled_db = sled::open(opt.kv_path)?;
    let sql = SQLiteDB::open(opt.sqlite_path)?;

    sql.query_row("SELECT 1 + 1", params![], |_| Ok(()))?;
    sled_db.insert(b"test", b"v1")?;
    sled_db.remove(b"test")?;

    Ok(())
}
