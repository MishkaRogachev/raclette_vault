#[cfg(test)]
mod tests {
    use crate::core::{key_pair, account};

    #[test]
    fn test_owned_from_keypair() {
        let keypair = key_pair::KeyPair::generate(42);
        let account = account::Account::owned_from_keypair(keypair.clone());

        assert_eq!(*account.address(), keypair.to_address());
    }

    #[test]
    fn test_watch_from_address() {
        let keypair = key_pair::KeyPair::generate(42);
        let address = keypair.to_address();
        let account = account::Account::watch_from_address(address.clone());

        assert_eq!(*account.address(), address);
    }
}
