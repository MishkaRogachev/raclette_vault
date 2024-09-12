use crate::{core::{key_pair::KeyPair, seed_phrase::SeedPhrase}, persistence};

const ROOT_KEYPAIR: &[u8] = b"root_keypair";

pub fn create_account(seed_phrase: &SeedPhrase, password: &str) -> anyhow::Result<()> {
    let path = std::env::current_dir()?;

    // NOTE: extra password may be used for seed_phrase -> keypair conversion
    let keypair = KeyPair::from_seed(seed_phrase.to_seed(""))?;
    let db = persistence::manage::open_database(path, keypair.to_address(), password)?;

    let serialized_keypair = serde_json::to_vec(&keypair)?;
    db.insert(ROOT_KEYPAIR, &serialized_keypair)?;

    Ok(())
}
