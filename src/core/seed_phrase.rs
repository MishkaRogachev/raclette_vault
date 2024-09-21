use bip39::{Language, Mnemonic, MnemonicType, Seed};

#[derive(Debug, Clone)]
pub struct SeedPhrase {
    pub mnemonic: Mnemonic,
}

impl SeedPhrase {
    pub fn generate(mtype: MnemonicType) -> Self {
        let mnemonic = Mnemonic::new(mtype, Language::English);
        Self { mnemonic }
    }

    pub fn switch_mnemonic_type(&mut self, mtype: MnemonicType) {
        self.mnemonic = Mnemonic::new(mtype, Language::English);
    }

    pub fn to_seed(&self, password: &str) -> Seed {
        Seed::new(&self.mnemonic, password)
    }

    pub fn from_string(s: &str) -> anyhow::Result<Self> {
        let mnemonic = Mnemonic::from_phrase(s, Language::English)?;
        Ok(Self { mnemonic })
    }

    pub fn from_words(words: Vec<String>) -> anyhow::Result<Self> {
        Self::from_string(words.join(" ").as_str())
    }

    pub fn get_words(&self) -> Vec<String> {
        self.mnemonic.to_string().split(' ').map(|s| s.to_string()).collect()
    }

    pub fn get_mnemonic_type(&self) -> MnemonicType {
        match self.mnemonic.to_string().split(' ').count() {
            12 => MnemonicType::Words12,
            24 => MnemonicType::Words24,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for SeedPhrase {
    fn eq(&self, other: &Self) -> bool {
        self.mnemonic.entropy() == other.mnemonic.entropy()
    }
}
