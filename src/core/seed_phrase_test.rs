#[cfg(test)]
mod tests {
    use crate::core::{key_pair::KeyPair, seed_phrase::SeedPhrase};

    #[test]
    fn test_seed_phrase_from_keypair_and_back() -> anyhow::Result<()> {
        let keypair = KeyPair::new();
        let seed_phrase = SeedPhrase::from_keypair(&keypair)?;

        assert_eq!(seed_phrase.mnemonic.language(), bip39::Language::English);
        assert_eq!(seed_phrase.mnemonic.word_count(), 24);

        let keypair_back = seed_phrase.to_keypair()?;
        assert_eq!(keypair, keypair_back);
        Ok(())
    }
}
