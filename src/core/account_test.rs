#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::core::{key_pair::KeyPair, account::Account};

    #[test]
    fn test_owned_from_keypair() {
        let keypair = KeyPair::generate(42);
        let account = Account::owned_from_keypair(keypair.clone());

        assert_eq!(*account.address(), keypair.to_address());
    }

    #[test]
    fn test_watch_from_address() {
        let keypair = KeyPair::generate(42);
        let address = keypair.to_address();
        let account = Account::watch_from_address(address.clone());

        assert_eq!(*account.address(), address);
    }

    #[test_case(Account::owned_from_keypair(KeyPair::generate(42)); "Ownded account")]
    #[test_case(Account::watch_from_address(KeyPair::generate(13).to_address()); "Watch account")]

    fn test_serialize_and_deserialize(account: Account) -> anyhow::Result<()> {
        let serialized = serde_json::to_string(&account)?;
        let deserialized: Account = serde_json::from_str(&serialized)?;

        assert_eq!(account, deserialized);
        Ok(())
    }
}
