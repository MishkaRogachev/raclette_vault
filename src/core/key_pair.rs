use secp256k1::{PublicKey,SecretKey,rand::{rngs,SeedableRng}};

#[derive(Clone, Debug)]
pub struct KeyPair {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
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
