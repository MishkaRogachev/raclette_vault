use std::fs;
use std::path::{Path, PathBuf};
use web3::types::Address;
use sled;
use anyhow::{Result, Context};

use super::db::Db;

pub const ACCOUNTS_DIR: &str = "raclette_accounts/";

pub fn open_database(path: PathBuf, address: Address, password: &str) -> Result<Db> {
    let db_path = db_path(&path, address)?;
    let config = sled::Config::new().path(db_path);
    let db = config.open().context("Failed to open database")?;
    Db::open(db, password).context("Failed to start database")
}

pub fn remove_database(path: PathBuf, address: Address) -> Result<()> {
    let db_path = db_path(&path, address)?;
    fs::remove_dir_all(db_path).context("Failed to remove database")
}

fn db_path(base_path: &Path, address: Address) -> Result<PathBuf> {
    let mut accounts_path = base_path.join(ACCOUNTS_DIR);

    if accounts_path.exists() {
        if !accounts_path.is_dir() {
            return Err(anyhow::anyhow!("'accounts' path exists but is not a directory"));
        }
    } else {
        fs::create_dir_all(&accounts_path).context("Failed to create accounts directory")?;
    }

    accounts_path.push(format!("{:?}", address));
    Ok(accounts_path)
}
