use secp256k1::rand::SeedableRng;

const SECRET_KEY_LEN: usize = 32;
const PUBLIC_KEY_LEN: usize = 33;

const ERR_SECRET_KEY_LEN: &str = "SecretKey must be 32 bytes";
const ERR_PUBLIC_KEY_LEN: &str = "PublicKey must be 33 bytes";
const ERR_SECRET_KEY_CONVERT: &str = "Failed to convert secret_key";
const ERR_PUBLIC_KEY_CONVERT: &str = "Failed to convert public_key";

pub struct KeyPair {
    pub secret_key: [u8; SECRET_KEY_LEN],
    pub public_key: [u8; PUBLIC_KEY_LEN],
}

impl KeyPair {
    pub fn new(state: u64) -> Self {
        let secp = secp256k1::Secp256k1::new();
        let mut rng = secp256k1::rand::rngs::StdRng::seed_from_u64(state);
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);

        Self {
            secret_key: secret_key[..].try_into().expect("SecretKey must be 32 bytes"),
            public_key: public_key.serialize(),
        }
    }

    pub fn to_address(&self) -> web3::types::Address {
        let public_key = &self.public_key;
        let hash = web3::signing::keccak256(&public_key[1..]);
        let address = &hash[12..];
        web3::types::Address::from_slice(address)
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
            return Err(serde::de::Error::custom(ERR_SECRET_KEY_LEN));
        }
        if public_key.len() != PUBLIC_KEY_LEN {
            return Err(serde::de::Error::custom(ERR_PUBLIC_KEY_LEN));
        }

        Ok(KeyPair {
            secret_key: secret_key.try_into().expect(ERR_SECRET_KEY_CONVERT),
            public_key: public_key.try_into().expect(ERR_PUBLIC_KEY_CONVERT),
        })
    }
}