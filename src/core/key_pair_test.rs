#[cfg(test)]
mod tests {
    use crate::core::key_pair::KeyPair;
    use regex::Regex;

    #[test]
    fn test_generate_keypair() {
        let keypair = KeyPair::new(42);

        assert_eq!(keypair.public_key.serialize().len(), 33, "Public key length mismatch");
        assert_eq!(keypair.secret_key[..].len(), 32, "Secret key length mismatch");

        let secret_key_str = keypair.secret_key.to_string();
        let public_key_str = keypair.public_key.to_string();

        let hex_regex = Regex::new(r"^[0-9a-f]{64}$").unwrap(); // Secret key should be 64 lowercase hex chars
        let public_key_regex = Regex::new(r"^[0-9a-f]{66}$|^[0-9a-f]{130}$").unwrap(); // Public key in compressed (66) or uncompressed (130) form

        assert!(hex_regex.is_match(&secret_key_str), "Invalid secret key format: {}", secret_key_str);
        assert!(public_key_regex.is_match(&public_key_str), "Invalid public key format: {}", public_key_str);
    }

    #[test]
    fn test_to_account() {
        let keypair = KeyPair::new(42);
        let address = keypair.to_address();

        assert_eq!(address.0.len(), 20, "Address length mismatch");
    }
}