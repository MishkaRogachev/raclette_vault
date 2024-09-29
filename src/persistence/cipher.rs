use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
use sha3::{Digest, Sha3_256};
use rand::Rng;
use anyhow::{Result, anyhow};

pub struct Cipher {
    cipher: Aes256Gcm,
}

impl Cipher {
    pub fn new_from_hash(hash: [u8; 32]) -> Self {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&hash));
        Self { cipher }
    }

    pub fn new_from_password(password: &str) -> Self {
        let hash = hash_password(password);
        Cipher::new_from_hash(hash)
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let nonce_bytes: [u8; 12] = rand::thread_rng().gen();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self.cipher.encrypt(nonce, plaintext)
            .map_err(|e| anyhow!("Encryption error: {:?}", e))?;
        Ok([nonce.as_slice(), ciphertext.as_slice()].concat())
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let (nonce, ciphertext) = ciphertext.split_at(12);
        let plaintext = self.cipher.decrypt(Nonce::from_slice(nonce), ciphertext)
            .map_err(|e| anyhow!("Decryption error: {:?}", e))?;
        Ok(plaintext)
    }
}

pub fn hash_password(password: &str) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

#[allow(dead_code)]
pub fn generate_random_hash() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill(&mut key);
    key
}
