use std::fs;
use std::path::{Path, PathBuf};
use web3::types::Address;
use sled;

use super::db::Db;

pub const ACCOUNTS_DIR: &str = "raclette_accounts/";

const ERR_ACCOUNTS_PATH_IS_NOT_DIR: &str  = "Accounts directory is not a directory";

pub fn open_database(path: &PathBuf, address: Address, password: &str) -> anyhow::Result<Db> {
    let db_path = db_path(path, address)?;
    let config = sled::Config::new().path(db_path);
    let db = config.open()?;
    Db::open(db, password)
}

pub fn remove_database(path: &PathBuf, address: Address) -> anyhow::Result<()> {
    let db_path = db_path(path, address)?;
    Ok(fs::remove_dir_all(db_path)?)
}

pub fn list_databases(path: &PathBuf) -> anyhow::Result<Vec<Address>> {
    let accounts_dir = path.join(ACCOUNTS_DIR);

    if !accounts_dir.exists() {
        return Ok(vec![]);
    }

    if !accounts_dir.is_dir() {
        return Err(anyhow::anyhow!(ERR_ACCOUNTS_PATH_IS_NOT_DIR));
    }

    let addresses: Vec<Address> = fs::read_dir(&accounts_dir)?
        .filter_map(|entry| {
            entry.ok().and_then(|entry| {
                let path = entry.path();
                if path.is_dir() {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .and_then(|folder_name| folder_name.parse::<Address>().ok())
                } else {
                    None
                }
            })
        })
        .collect();
    Ok(addresses)
}

pub fn remove_all_databases(path: &Path) -> anyhow::Result<()> {
    let accounts_dir = path.join(ACCOUNTS_DIR);

    if accounts_dir.exists() {
        fs::remove_dir_all(accounts_dir)?;
    }

    Ok(())
}

fn db_path(base_path: &Path, address: Address) -> anyhow::Result<PathBuf> {
    let mut accounts_path = base_path.join(ACCOUNTS_DIR);

    if accounts_path.exists() {
        if !accounts_path.is_dir() {
            return Err(anyhow::anyhow!(ERR_ACCOUNTS_PATH_IS_NOT_DIR));
        }
    } else {
        fs::create_dir_all(&accounts_path)?;
    }

    accounts_path.push(format!("{:?}", address));
    Ok(accounts_path)
}
