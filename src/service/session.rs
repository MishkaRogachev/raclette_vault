use std::sync::Arc;
use web3::signing::SecretKey;

use crate::core::{eth_chain, key_pair::KeyPair, seed_phrase::SeedPhrase};
use crate::persistence::{db::Db, manage};
use crate::utils;

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

        let account = keypair.get_eth_address();
        let db = manage::open_database(&utils::app_data_path()?, account, password)?;

        db.save_seed_phrase(seed_phrase)?;
        db.save_keypair(&keypair)?;
        db.save_active_networks(&eth_chain::MAINNET_CHAINS)?;

        Ok(Session {
            account,
            db: Arc::new(db),
        })
    }

    pub fn login(account: web3::types::Address, password: &str) -> anyhow::Result<Self> {
        let db = manage::open_database(&utils::app_data_path()?, account, password)?;
        if db.get_keypair().is_err() {
            return Err(anyhow::anyhow!(ERR_WRONG_PASSWORD_PROVIDED));
        }

        Ok(Session {
            account,
            db: Arc::new(db),
        })
    }

    pub fn list_accounts() -> anyhow::Result<Vec<web3::types::Address>> {
        manage::list_databases(&utils::app_data_path()?)
    }

    pub fn remove_account(account: web3::types::Address) -> anyhow::Result<()> {
        manage::remove_database(&utils::app_data_path()?, account)
    }

    pub fn delete_account(&self) -> anyhow::Result<()> {
        Self::remove_account(self.account)
    }

    pub fn get_secret_key(&self) -> anyhow::Result<SecretKey> {
        let keypair = self.db.get_keypair()?;
        SecretKey::from_slice(keypair.secret_key.as_slice())
            .map_err(|_| anyhow::anyhow!("Failed to get secret key"))
    }
}
