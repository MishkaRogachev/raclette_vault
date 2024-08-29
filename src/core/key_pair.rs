use anyhow::Context;
use serde::{Serialize, Deserialize};
use secp256k1::{PublicKey,SecretKey,rand::{rngs,SeedableRng}};

#[derive(Clone, Debug, PartialEq)]
pub struct KeyPair {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize)]
struct KeyPairHelper {
    secret_key: String,
    public_key: String,
}

impl KeyPair {
    pub fn generate(state: u64) -> Self {
        let secp = secp256k1::Secp256k1::new();
        let mut rng = rngs::StdRng::seed_from_u64(state); // TODO: use rand::Rng;
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);

        Self {
            secret_key,
            public_key,
        }
    }

    pub fn to_address(&self) -> web3::types::Address {
        let public_key = self.public_key.serialize_uncompressed();
        let hash = web3::signing::keccak256(&public_key[1..]);
        let address = &hash[12..];
        web3::types::Address::from_slice(address)
    }
}

impl Serialize for KeyPair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let helper = KeyPairHelper {
            secret_key: hex::encode(&self.secret_key[..]),
            public_key: hex::encode(self.public_key.serialize()),
        };

        helper.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for KeyPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = KeyPairHelper::deserialize(deserializer)
            .map_err(serde::de::Error::custom)?;

        let secret_key_bytes = hex::decode(&helper.secret_key)
            .context("Failed to decode secret key from hex")
            .map_err(serde::de::Error::custom)?;
        let public_key_bytes = hex::decode(&helper.public_key)
            .context("Failed to decode public key from hex")
            .map_err(serde::de::Error::custom)?;

        let secret_key = SecretKey::from_slice(&secret_key_bytes)
            .context("Failed to create SecretKey from bytes")
            .map_err(serde::de::Error::custom)?;
        let public_key = PublicKey::from_slice(&public_key_bytes)
            .context("Failed to create PublicKey from bytes")
            .map_err(serde::de::Error::custom)?;

        Ok(KeyPair { secret_key, public_key })
    }
}
