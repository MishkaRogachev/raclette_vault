#[cfg(test)]
mod tests {
    use crate::core::{key_pair::KeyPair, seed_phrase::SeedPhrase};

    #[test]
    fn test_seed_phrase_from_keypair_and_back() -> anyhow::Result<()> {
        let keypair = KeyPair::new();
        let seed_phrase = SeedPhrase::from_keypair(&keypair)?;

        assert_eq!(seed_phrase.mnemonic.language(), bip39::Language::English);
        assert_eq!(seed_phrase.mnemonic.phrase().split(' ').count(), 24);

        assert_eq!( bip39::Mnemonic::validate(&seed_phrase.to_string(), bip39::Language::English).is_ok(), true);

        let keypair_back = seed_phrase.to_keypair()?;
        assert_eq!(keypair, keypair_back);
        Ok(())
    }

    #[test]
    fn test_generate_seed_phrase_to_words_and_back() -> anyhow::Result<()> {
        let seed_phrase = SeedPhrase::generate(bip39::MnemonicType::Words12);
        assert_eq!(seed_phrase.mnemonic.language(), bip39::Language::English);

        let words = seed_phrase.to_words();
        assert_eq!(words.len(), 12);

        let seed_phrase_back = SeedPhrase::from_words(words)?;
        assert_eq!(seed_phrase, seed_phrase_back);
        Ok(())
    }
}
