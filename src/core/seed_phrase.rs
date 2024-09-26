use bip39::Mnemonic;
use zeroize::Zeroizing;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WordCount {
    Words12 = 12,
    Words18 = 18,
    Words24 = 24,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeedPhrase {
    pub mnemonic: Mnemonic,
}

impl SeedPhrase {
    pub fn generate(word_count: WordCount) -> anyhow::Result<Self> {
        let mnemonic = Mnemonic::generate(word_count as usize)?;
        Ok(Self { mnemonic })
    }

    pub fn to_seed(&self, password: &str) -> [u8; 64] {
        self.mnemonic.to_seed(password)
    }

    pub fn from_string(s: &str) -> anyhow::Result<Self> {
        let mnemonic = Mnemonic::parse(s)?;
        Ok(Self { mnemonic })
    }

    pub fn from_words(words: Vec<String>) -> anyhow::Result<Self> {
        Self::from_string(words.join(" ").as_str())
    }

    pub fn get_words(&self) -> Vec<String> {
        self.mnemonic.to_string().split(' ').map(|s| s.to_string()).collect()
    }

    pub fn get_words_zeroizing(&self) -> Vec<Zeroizing<String>> {
        self.get_words().iter().map(|w| Zeroizing::new(w.clone())).collect()
    }
}
