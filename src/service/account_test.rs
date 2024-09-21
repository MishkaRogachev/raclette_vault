#[cfg(test)]
mod tests {
    use test_case::{test_matrix, test_case};
    use bip39::MnemonicType;
    use crate::core::{key_pair::KeyPair, seed_phrase::SeedPhrase};
    use super::super::account::Account;

    #[test_matrix(
        [MnemonicType::Words12, MnemonicType::Words24],
        ["12345678", ""]
    )]
    fn test_account_flow(mtype: bip39::MnemonicType, password: &str) -> anyhow::Result<()> {
        let seed_phrase = SeedPhrase::generate(mtype);
        let keypair = KeyPair::from_seed(seed_phrase.to_seed(""))?;
        keypair.validate()?;

        let address = {
            let account = Account::create(&seed_phrase, password)?;
            account.address
        };

        // Try wrong password
        assert!(Account::login(address, "wrong_password").is_err());

        let account = Account::login(address, password)?;
        let keypair_back = account.get_keypair()?;
        keypair_back.validate()?;
        assert_eq!(keypair, keypair_back);

        let accounts = Account::list_accounts()?;
        assert!(accounts.contains(&address));

        // Access seed phrase
        let seed_phrase_back = account.get_seed_phrase()?;
        assert_eq!(seed_phrase, seed_phrase_back);

        // Delete seed phrase
        account.delete_seed_phrase()?;
        assert!(account.get_seed_phrase().is_err());

        // Remove account
        account.delete_account()?;

        let accounts = Account::list_accounts()?;
        assert!(!accounts.contains(&address));

        Ok(())
    }

    #[test_case(bip39::MnemonicType::Words12)]
    #[test_case(bip39::MnemonicType::Words24)]
    fn test_restore_from_seed_phrase(mtype: bip39::MnemonicType) -> anyhow::Result<()> {
        let seed_phrase = SeedPhrase::generate(mtype);
        let keypair = KeyPair::from_seed(seed_phrase.to_seed(""))?;
        keypair.validate()?;

        let address = {
            let account = Account::create(&seed_phrase, "")?;
            account.address
        };

        let seed_phrase_back = SeedPhrase::from_words(seed_phrase.get_words())?;
        let account = Account::create(&seed_phrase_back, "")?;
        assert_eq!(account.address, address);

        Ok(())
    }
}
