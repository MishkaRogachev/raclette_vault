use std::sync::Arc;

use crate::core::{chain, key_pair::KeyPair, seed_phrase::SeedPhrase};
use crate::persistence::{db::Db, manage};

const ERR_WRONG_PASSWORD_PROVIDED: &str = "Wrong password provided";

#[derive(Clone)]
pub struct Session {
    pub account: web3::types::Address,
    pub db: Arc<Db>
}

impl Session {
    pub fn create_account(seed_phrase: &SeedPhrase, password: &str) -> anyhow::Result<Self> {
        // NOTE: extra password may be used for seed_phrase -> keypair conversion
        let keypair = KeyPair::from_seed(seed_phrase.to_seed(""))?;
        keypair.validate()?;

        let account = keypair.get_address();
        let db = manage::open_database(&db_path()?, account, password)?;

        db.save_seed_phrase(seed_phrase)?;
        db.save_keypair(&keypair)?;
        db.save_active_networks(&chain::MAINNET_CHAINS)?;

        Ok(Session {
            account,
            db: Arc::new(db),
        })
    }

    pub fn login(account: web3::types::Address, password: &str) -> anyhow::Result<Self> {
        let db = manage::open_database(&db_path()?, account, password)?;
        if db.get_keypair().is_err() {
            return Err(anyhow::anyhow!(ERR_WRONG_PASSWORD_PROVIDED));
        }

        Ok(Session {
            account,
            db: Arc::new(db),
        })
    }

    pub fn list_accounts() -> anyhow::Result<Vec<web3::types::Address>> {
        manage::list_databases(&db_path()?)
    }

    pub fn remove_account(account: web3::types::Address) -> anyhow::Result<()> {
        manage::remove_database(&db_path()?, account)
    }

    pub fn delete_account(&self) -> anyhow::Result<()> {
        Self::remove_account(self.account)
    }
}

fn db_path() -> std::io::Result<std::path::PathBuf> {
    if cfg!(test) {
        Ok(std::env::temp_dir())
    } else if cfg!(debug_assertions) {
        Ok(std::env::current_dir()?)
    } else {
        if let Some(proj_dirs) = directories::ProjectDirs::from(
            "com", "raclettevault", "RacletteVault") {
            Ok(proj_dirs.data_dir().to_path_buf())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unable to determine a proper data directory",
            ))
        }
    }
}
