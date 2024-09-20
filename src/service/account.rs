use std::sync::Arc;

use crate::{core::{key_pair::KeyPair, seed_phrase::SeedPhrase}, persistence};

const ROOT_KEYPAIR: &[u8] = b"root_keypair";
const ROOT_SEED_PHRASE: &[u8] = b"root_seed_phrase";

const ERR_SEED_PHRASE_NOT_FOUND: &str = "Seed phrase not found";
const ERR_ACCOUNT_NOT_FOUND: &str = "Account not found";
const ERR_WRONG_PASSWORD_PROVIDED: &str = "Wrong password provided";

#[derive(Clone)]
pub struct Account {
    pub address: web3::types::Address,
    db: Arc<persistence::db::Db>
}

impl Account {
    pub fn create(seed_phrase: &SeedPhrase, password: &str) -> anyhow::Result<Self> {
        // NOTE: extra password may be used for seed_phrase -> keypair conversion
        let keypair = KeyPair::from_seed(seed_phrase.to_seed(""))?;
        keypair.validate()?;

        let address = keypair.get_address();
        let db = persistence::manage::open_database(&db_path()?, address, password)?;

        let words = seed_phrase.get_words();
        let serialized_seed_phrase = serde_json::to_vec(&words)?;
        db.insert(ROOT_SEED_PHRASE, &serialized_seed_phrase)?;

        let serialized_keypair = serde_json::to_vec(&keypair)?;
        db.insert(ROOT_KEYPAIR, &serialized_keypair)?;

        Ok(Account {
            address,
            db: Arc::new(db),
        })
    }

    pub fn login(address: web3::types::Address, password: &str) -> anyhow::Result<Self> {
        let db = persistence::manage::open_database(&db_path()?, address, password)?;
        let account = Account {
            address,
            db: Arc::new(db),
        };

        if account.get_keypair().is_err() {
            return Err(anyhow::anyhow!(ERR_WRONG_PASSWORD_PROVIDED));
        }

        Ok(account)
    }

    pub fn list_accounts() -> anyhow::Result<Vec<web3::types::Address>> {
        persistence::manage::list_databases(&db_path()?)
    }

    pub fn remove_account(address: web3::types::Address) -> anyhow::Result<()> {
        persistence::manage::remove_database(&db_path()?, address)
    }

    pub fn get_seed_phrase(&self) -> anyhow::Result<SeedPhrase> {
        let serialized_seed_phrase: Option<Vec<u8>> = self.db.get(ROOT_SEED_PHRASE)?;
        if let Some(serialized_seed_phrase) = serialized_seed_phrase {
            let words: Vec<String> = serde_json::from_slice(&serialized_seed_phrase)?;
            return SeedPhrase::from_words(words);
        }
        Err(anyhow::anyhow!(ERR_SEED_PHRASE_NOT_FOUND))
    }

    pub fn get_keypair(&self) -> anyhow::Result<KeyPair> {
        let serialized_keypair: Option<Vec<u8>> = self.db.get(ROOT_KEYPAIR)?;
        if let Some(serialized_keypair) = serialized_keypair {
            let keypair: KeyPair = serde_json::from_slice(&serialized_keypair)?;
            keypair.validate()?;
            return Ok(keypair);
        }
        Err(anyhow::anyhow!(ERR_ACCOUNT_NOT_FOUND))
    }

    pub fn delete_seed_phrase(&self) -> anyhow::Result<()> {
        match self.db.remove(ROOT_SEED_PHRASE) {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow::anyhow!(ERR_SEED_PHRASE_NOT_FOUND))
        }
    }

    pub fn delete_account(&self) -> anyhow::Result<()> {
        Self::remove_account(self.address)
    }
}

fn db_path() -> std::io::Result<std::path::PathBuf> {
    if cfg!(test) {
        Ok(std::env::temp_dir())
    } else {
        std::env::current_dir()
    }
}
