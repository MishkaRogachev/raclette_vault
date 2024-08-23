use secp256k1::{PublicKey,SecretKey,rand::{rngs,SeedableRng}};

pub struct KeyPair {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}

impl KeyPair {
    pub fn generate(state: u64) -> Self {
        let secp = secp256k1::Secp256k1::new();
        let mut rng = rngs::StdRng::seed_from_u64(state);
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);

        Self {
            secret_key,
            public_key,
        }
    }
}
