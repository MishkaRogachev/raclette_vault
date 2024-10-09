use crate::core::{key_pair::KeyPair, seed_phrase::SeedPhrase};
use super::db::Db;

const ROOT_KEYPAIR: &[u8] = b"root_keypair";
const ROOT_SEED_PHRASE: &[u8] = b"root_seed_phrase";

const ERR_SEED_PHRASE_NOT_FOUND: &str = "Seed phrase not found";
const ERR_KEYPAIR_NOT_FOUND: &str = "Keypair not found";

impl Db {
    pub fn save_seed_phrase(&self, seed_phrase: &SeedPhrase) -> anyhow::Result<()> {
        let words = seed_phrase.get_words();
        let serialized_seed_phrase = serde_json::to_vec(&words)?;
        self.upsert(ROOT_SEED_PHRASE, &serialized_seed_phrase, true)
    }

    pub fn get_seed_phrase(&self) -> anyhow::Result<SeedPhrase> {
        let serialized_seed_phrase: Option<Vec<u8>> = self.get(ROOT_SEED_PHRASE, true)?;
        if let Some(serialized_seed_phrase) = serialized_seed_phrase {
            let words: Vec<String> = serde_json::from_slice(&serialized_seed_phrase)?;
            return SeedPhrase::from_words(words);
        }
        Err(anyhow::anyhow!(ERR_SEED_PHRASE_NOT_FOUND))
    }

    pub fn delete_seed_phrase(&self) -> anyhow::Result<()> {
        match self.remove(ROOT_SEED_PHRASE) {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow::anyhow!(ERR_SEED_PHRASE_NOT_FOUND))
        }
    }

    pub fn save_keypair(&self, keypair: &KeyPair) -> anyhow::Result<()> {
        let serialized_keypair = serde_json::to_vec(&keypair)?;
        self.upsert(ROOT_KEYPAIR, &serialized_keypair, true)
    }

    pub fn get_keypair(&self) -> anyhow::Result<KeyPair> {
        let serialized_keypair: Option<Vec<u8>> = self.get(ROOT_KEYPAIR, true)?;
        if let Some(serialized_keypair) = serialized_keypair {
            let keypair: KeyPair = serde_json::from_slice(&serialized_keypair)?;
            keypair.validate()?;
            return Ok(keypair);
        }
        Err(anyhow::anyhow!(ERR_KEYPAIR_NOT_FOUND))
    }
}
