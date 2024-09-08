#[cfg(test)]
mod tests {
    use super::super::cipher;

    #[test]
    fn test_hash_password() {
        let password = "12345678";
        let hash = cipher::hash_password(password);

        assert_eq!(hash.len(), 32);

        let hash2 = cipher::hash_password(password);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_encrypt_and_decrypt() -> anyhow::Result<()> {
        let cipher = cipher::Cipher::new_from_hash(cipher::generate_random_hash());

        let plaintext = b"how are you doing?";
        let ciphertext = cipher.encrypt(plaintext)?;

        assert_ne!(plaintext.to_vec(), ciphertext);
        let decrypted_plaintext = cipher.decrypt(&ciphertext)?;

        assert_eq!(plaintext.to_vec(), decrypted_plaintext);
        Ok(())
    }

    #[test]
    fn test_try_to_decrypt_with_wrong_key() -> anyhow::Result<()> {
        let cipher1 = cipher::Cipher::new_from_password("12345678");
        let cipher2 = cipher::Cipher::new_from_password("01234567");

        let plaintext = b"how are you doing?";
        let ciphertext = cipher1.encrypt(plaintext)?;

        assert!(cipher2.decrypt(&ciphertext).is_err());
        Ok(())
    }
}
