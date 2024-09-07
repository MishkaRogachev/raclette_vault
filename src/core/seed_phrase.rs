use bip39::{Language, Mnemonic, MnemonicType, Seed};

#[derive(Debug)]
pub struct SeedPhrase {
    pub mnemonic: Mnemonic,
}

impl SeedPhrase {
    pub fn generate(mtype: MnemonicType) -> Self {
        let mnemonic = Mnemonic::new(mtype, Language::English);
        Self { mnemonic }
    }

    pub fn to_seed(&self, password: &str) -> Seed {
        Seed::new(&self.mnemonic, password)
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
        self.mnemonic.entropy() == other.mnemonic.entropy()
    }
}
