#[cfg(test)]
mod tests {
    use crate::core::key_pair;
    use regex::Regex;

    #[test]
    fn test_generate_keypair() {
        let keypair = key_pair::KeyPair::new();

        assert_eq!(keypair.public_key.len(), key_pair::PUBLIC_KEY_LEN, "Public key length mismatch");
        assert_eq!(keypair.secret_key.len(), key_pair::SECRET_KEY_LEN, "Secret key length mismatch");

        let secret_key_str = hex::encode(&keypair.secret_key);
        let public_key_str = hex::encode(&keypair.public_key);

        let secret_key_regex = Regex::new(r"^[0-9a-f]{64}$").unwrap();
        let public_key_regex = Regex::new(r"^[0-9a-f]{66}$|^[0-9a-f]{130}$").unwrap();

        assert!(secret_key_regex.is_match(&secret_key_str), "Invalid secret key format: {}", secret_key_str);
        assert!(public_key_regex.is_match(&public_key_str), "Invalid public key format: {}", public_key_str);
    }

    #[test]
    fn test_to_account() {
        let keypair = key_pair::KeyPair::new();
        let address = keypair.to_address();

        assert_eq!(address.0.len(), 20, "Address length mismatch");
    }
}
