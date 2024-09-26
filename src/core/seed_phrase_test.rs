#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::super::seed_phrase::{SeedPhrase, WordCount};

    #[test_case(WordCount::Words12)]
    #[test_case(WordCount::Words24)]
    fn test_generate_seed_phrase_to_words_and_back(word_count: WordCount) -> anyhow::Result<()> {
        let seed_phrase = SeedPhrase::generate(word_count)?;

        let words = seed_phrase.get_words();
        assert_eq!(words.len(), word_count as usize);

        let seed_phrase_back = SeedPhrase::from_words(words)?;
        assert_eq!(seed_phrase, seed_phrase_back);
        Ok(())
    }
}
