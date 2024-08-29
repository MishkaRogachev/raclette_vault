use sled::IVec;
use serde::{Serialize, Deserialize};
use bincode;
use anyhow::Result;

use super::cipher::Cipher;

pub struct Db {
    db: sled::Db,
    cipher: Cipher,
}

impl Db {
    pub fn open(path: &str, password: &str) -> Result<Self> {
        let db = sled::open(path)?;
        let cipher = Cipher::new_from_password(password);

        Ok(Self {
            db,
            cipher,
        })
    }

    pub fn insert<V>(&self, key: &[u8], value: &V) -> Result<()>
    where
        V: Serialize,
    {
        let serialized_value = bincode::serialize(value)?;
        let encrypted_value = self.cipher.encrypt(&serialized_value)?;
        self.db.insert(key, encrypted_value)?;
        Ok(())
    }

    pub fn get<V>(&self, key: &[u8]) -> Result<Option<V>>
    where
        V: for<'de> Deserialize<'de>,
    {
        if let Some(encrypted_value) = self.db.get(key)? {
            let decrypted_value = self.cipher.decrypt(&encrypted_value)?;
            let deserialized_value = bincode::deserialize(&decrypted_value)?;
            Ok(Some(deserialized_value))
        } else {
            Ok(None)
        }
    }

    pub fn remove(&self, key: &[u8]) -> Result<Option<IVec>> {
        self.db.remove(key).map_err(Into::into)
    }
}
