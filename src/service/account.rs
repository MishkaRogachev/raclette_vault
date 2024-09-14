use crate::{core::{key_pair::KeyPair, seed_phrase::SeedPhrase}, persistence};

const ROOT_KEYPAIR: &[u8] = b"root_keypair";

pub struct Account {
    db: persistence::db::Db
}

impl Account {
    pub fn create(seed_phrase: &SeedPhrase, password: &str) -> anyhow::Result<Self> {
        let path = std::env::current_dir()?;

        // NOTE: extra password may be used for seed_phrase -> keypair conversion
        let keypair = KeyPair::from_seed(seed_phrase.to_seed(""))?;
        keypair.validate()?;

        let db = persistence::manage::open_database(path, keypair.to_address(), password)?;
        let serialized_keypair = serde_json::to_vec(&keypair)?;
        db.insert(ROOT_KEYPAIR, &serialized_keypair)?;
        Ok(Account{ db })
    }

    pub fn login(address: web3::types::Address, password: &str) -> anyhow::Result<Self> {
        let path = std::env::current_dir()?;
        let db = persistence::manage::open_database(path, address, password)?;
        Ok(Account{ db })
    }
}

