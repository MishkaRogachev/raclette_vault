#[cfg(test)]
mod tests {
    use test_case::{test_matrix, test_case};
    use crate::core::{key_pair::KeyPair, seed_phrase::{WordCount, SeedPhrase}};
    use super::super::session::Session;

    #[test_matrix(
        [WordCount::Words12, WordCount::Words24],
        ["12345678", ""]
    )]
    fn test_session_flow(word_count: WordCount, password: &str) -> anyhow::Result<()> {
        let seed_phrase = SeedPhrase::generate(word_count)?;
        let keypair = KeyPair::from_seed(seed_phrase.to_seed(""))?;
        keypair.validate()?;

        let account = {
            let session = Session::create_account(&seed_phrase, password)?;
            session.account
        };

        // Try wrong password
        assert!(Session::login(account, "wrong_password").is_err());

        let session = Session::login(account, password)?;
        let keypair_back = session.db.get_keypair()?;
        keypair_back.validate()?;
        assert_eq!(keypair, keypair_back);

        let accounts = Session::list_accounts()?;
        assert!(accounts.contains(&account));

        // Access seed phrase
        let seed_phrase_back = session.db.get_seed_phrase()?;
        assert_eq!(seed_phrase, seed_phrase_back);

        // Delete seed phrase
        session.db.delete_seed_phrase()?;
        assert!(session.db.get_seed_phrase().is_err());

        // Remove account
        session.delete_account()?;

        let accounts = Session::list_accounts()?;
        assert!(!accounts.contains(&account));

        Ok(())
    }

    #[test_case(WordCount::Words12)]
    #[test_case(WordCount::Words24)]
    fn test_restore_from_seed_phrase(word_count: WordCount) -> anyhow::Result<()> {
        let seed_phrase = SeedPhrase::generate(word_count)?;
        let keypair = KeyPair::from_seed(seed_phrase.to_seed(""))?;
        keypair.validate()?;

        let account = {
            let session = Session::create_account(&seed_phrase, "")?;
            session.account
        };

        let seed_phrase_back = SeedPhrase::from_words(seed_phrase.get_words())?;
        let session = Session::create_account(&seed_phrase_back, "")?;
        assert_eq!(session.account, account);

        Ok(())
    }
}
