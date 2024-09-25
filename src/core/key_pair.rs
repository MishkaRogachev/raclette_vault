use std::ops::Deref;
use bip39::Seed;
use secp256k1::Secp256k1;
use zeroize::Zeroizing;

pub const SECRET_KEY_LEN: usize = secp256k1::constants::SECRET_KEY_SIZE;
pub const PUBLIC_KEY_LEN: usize = secp256k1::constants::PUBLIC_KEY_SIZE;

const BIP44_ETH_DERIVATION_PATH: &str = "m/44'/60'/0'/0/0";

const ERR_SECRET_KEY_CONVERT: &str = "Failed to convert secret_key";
const ERR_PUBLIC_KEY_CONVERT: &str = "Failed to convert public_key";
const ERR_PUBLIC_KEY_NOT_MATCH: &str = "Public key does not match secret key";

#[derive(Clone, Debug, PartialEq)]
pub struct KeyPair {
    pub secret_key: Zeroizing<[u8; SECRET_KEY_LEN]>,
    pub public_key: Zeroizing<[u8; PUBLIC_KEY_LEN]>,
}

impl KeyPair {
    pub fn from_seed(seed: Seed) -> anyhow::Result<Self> {
        let secp = Secp256k1::new();

        // Derive the extended private key from the seed
        let hd_key = hdkey::HDKey::from_master_seed(seed.as_bytes(), None)?;
        let derived_xprv = hd_key.derive(BIP44_ETH_DERIVATION_PATH)?;

        // Extract the private key from the derived key
        let secret_key_bytes = derived_xprv.private_key()
            .ok_or_else(|| anyhow::anyhow!("Failed to get private key"))?;
        let secret_key = secp256k1::SecretKey::from_slice(&secret_key_bytes)?;

        // Derive the public key from the private key
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);

        Ok(Self::from_secp256k1(secret_key, public_key))
    }

    pub fn from_secp256k1(secret_key: secp256k1::SecretKey, public_key: secp256k1::PublicKey) -> Self {
        Self {
            secret_key: Zeroizing::new(secret_key[..].try_into().expect(&err_secret_key_len())),
            public_key: Zeroizing::new(public_key.serialize())
        }
    }

    pub fn get_address(&self) -> web3::types::Address {
        let public_key = &self.public_key;
        let hash = web3::signing::keccak256(&public_key[1..]);
        let address = &hash[12..];
        web3::types::Address::from_slice(address)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        let secp = Secp256k1::new();
        let secret_key = secp256k1::SecretKey::from_slice(self.secret_key.deref())?;
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);

        if *self.public_key.deref() != public_key.serialize() {
            return Err(anyhow::anyhow!(ERR_PUBLIC_KEY_NOT_MATCH));
        }
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct KeyPairHelper {
    secret_key: String,
    public_key: String,
}

impl serde::Serialize for KeyPair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let helper = KeyPairHelper {
            secret_key: hex::encode(&self.secret_key),
            public_key: hex::encode(&self.public_key),
        };

        helper.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for KeyPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = KeyPairHelper::deserialize(deserializer)?;

        let secret_key = hex::decode(&helper.secret_key)
            .map_err(serde::de::Error::custom)?;
        let public_key = hex::decode(&helper.public_key)
            .map_err(serde::de::Error::custom)?;

        if secret_key.len() != SECRET_KEY_LEN {
            return Err(serde::de::Error::custom(err_secret_key_len()));
        }
        if public_key.len() != PUBLIC_KEY_LEN {
            return Err(serde::de::Error::custom(err_public_key_len()));
        }

        Ok(KeyPair {
            secret_key: Zeroizing::new(secret_key.try_into().expect(ERR_SECRET_KEY_CONVERT)),
            public_key: Zeroizing::new(public_key.try_into().expect(ERR_PUBLIC_KEY_CONVERT)),
        })
    }
}

fn err_secret_key_len() -> String {
    format!("SecretKey must be {} bytes", SECRET_KEY_LEN)
}

fn err_public_key_len() -> String {
    format!("PublicKey must be {} bytes", PUBLIC_KEY_LEN)
}
