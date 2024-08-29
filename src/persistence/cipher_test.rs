#[cfg(test)]
mod tests {
    use crate::persistence::cipher;

    #[test]
    fn test_hash_password() {
        let password = "12345678";
        let hash = cipher::hash_password(password);

        assert_eq!(hash.len(), 32);

        let hash2 = cipher::hash_password(password);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let cipher = cipher::Cipher::new_from_hash(cipher::generate_random_hash());

        let plaintext = b"how are you doing?";
        let ciphertext = cipher.encrypt(plaintext).expect("Encryption failed");

        assert_ne!(plaintext.to_vec(), ciphertext);
        let decrypted_plaintext = cipher.decrypt(&ciphertext).expect("Decryption failed");

        assert_eq!(plaintext.to_vec(), decrypted_plaintext);
    }

    #[test]
    fn test_try_to_decrypt_with_wrong_key() {
        let cipher1 = cipher::Cipher::new_from_password("12345678");
        let cipher2 = cipher::Cipher::new_from_password("01234567");

        let plaintext = b"how are you doing?";
        let ciphertext = cipher1.encrypt(plaintext).expect("Encryption failed");

        assert!(cipher2.decrypt(&ciphertext).is_err());
    }
}
