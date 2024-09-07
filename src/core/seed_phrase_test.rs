#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::super::seed_phrase::SeedPhrase;

    #[test_case(bip39::MnemonicType::Words12)]
    #[test_case(bip39::MnemonicType::Words24)]
    fn test_generate_seed_phrase_to_words_and_back(mtype: bip39::MnemonicType) -> anyhow::Result<()> {
        let seed_phrase = SeedPhrase::generate(mtype);

        let words = seed_phrase.to_words();
        assert_eq!(words.len(), mtype.word_count());

        let seed_phrase_back = SeedPhrase::from_words(words)?;
        assert_eq!(seed_phrase, seed_phrase_back);
        Ok(())
    }
}
