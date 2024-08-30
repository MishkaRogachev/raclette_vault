use secp256k1::rand::rngs::JitterRng;

pub const SECRET_KEY_LEN: usize = secp256k1::constants::SECRET_KEY_SIZE;
pub const PUBLIC_KEY_LEN: usize = secp256k1::constants::PUBLIC_KEY_SIZE;

const ERR_SECRET_KEY_CONVERT: &str = "Failed to convert secret_key";
const ERR_PUBLIC_KEY_CONVERT: &str = "Failed to convert public_key";

pub struct KeyPair {
    pub secret_key: [u8; SECRET_KEY_LEN],
    pub public_key: [u8; PUBLIC_KEY_LEN],
}

impl KeyPair {
    pub fn new(state: u64) -> Self {
        let secp = secp256k1::Secp256k1::new();
        let mut rng = JitterRng::new_with_timer(get_nstime);
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
            return Err(serde::de::Error::custom(err_secret_key_len()));
        }
        if public_key.len() != PUBLIC_KEY_LEN {
            return Err(serde::de::Error::custom(err_public_key_len()));
        }

        Ok(KeyPair {
            secret_key: secret_key.try_into().expect(ERR_SECRET_KEY_CONVERT),
            public_key: public_key.try_into().expect(ERR_PUBLIC_KEY_CONVERT),
        })
    }
}

fn get_nstime() -> u64 {
    let dur = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|_| { std::time::Duration::from_secs(0) });
    dur.as_secs() << 30 | dur.subsec_nanos() as u64
}

fn err_secret_key_len() -> String {
    format!("SecretKey must be {} bytes", SECRET_KEY_LEN)
}

fn err_public_key_len() -> String {
    format!("PublicKey must be {} bytes", PUBLIC_KEY_LEN)
}