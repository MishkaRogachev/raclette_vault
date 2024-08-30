use bip39::Mnemonic;

use super::key_pair::KeyPair;

pub struct SeedPhrase {
    pub mnemonic: Mnemonic,
}

impl SeedPhrase {
    pub fn from_keypair(keypair: &KeyPair) -> Self {
        let mnemonic = bip39::Mnemonic::from_entropy(&keypair.secret_key).unwrap();
        SeedPhrase { mnemonic }
    }

    pub fn to_keypair(&self) -> anyhow::Result<KeyPair> {
        let (entropy, entropy_len) = self.mnemonic.to_entropy_array();
        let secret_key = secp256k1::SecretKey::from_slice(&entropy[..entropy_len.min(32)])?;
        KeyPair::from_secret_key(secret_key)
    }
}
