use bip39::{Mnemonic, MnemonicType, Language};

use super::key_pair::KeyPair;

#[derive(Debug)]
pub struct SeedPhrase {
    pub mnemonic: Mnemonic,
}

impl SeedPhrase {
    pub fn generate(mtype: MnemonicType) -> Self {
        let mnemonic = Mnemonic::new(mtype, Language::English);
        Self { mnemonic }
    }

    pub fn from_keypair(keypair: &KeyPair) -> anyhow::Result<Self> {
        let mnemonic = bip39::Mnemonic::from_entropy(&keypair.secret_key, Language::English)?;
        Ok(Self { mnemonic })
    }

    pub fn to_keypair(&self) -> anyhow::Result<KeyPair> {
        let entropy = self.mnemonic.entropy();
        let secret_key = secp256k1::SecretKey::from_slice(entropy)?;
        KeyPair::from_secret_key(secret_key)
    }

    pub fn from_string(s: &str) -> anyhow::Result<Self> {
        let mnemonic = Mnemonic::from_phrase(s, Language::English)?;
        Ok(Self { mnemonic })
    }

    pub fn to_string(&self) -> String {
        self.mnemonic.to_string()
    }

    pub fn from_words(words: Vec<String>) -> anyhow::Result<Self> {
        Self::from_string(words.join(" ").as_str())
    }

    pub fn to_words(&self) -> Vec<String> {
        self.mnemonic.to_string().split(' ').map(|s| s.to_string()).collect()
    }
}

impl PartialEq for SeedPhrase {
    fn eq(&self, other: &Self) -> bool {
        (self.mnemonic.entropy() == other.mnemonic.entropy())
    }
}
