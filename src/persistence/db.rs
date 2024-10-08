use serde::{Serialize, Deserialize};
use serde_json;
use anyhow::Result;

use super::cipher::Cipher;

pub struct Db {
    db: sled::Db,
    cipher: Cipher,
}

impl Db {
    pub fn open(db: sled::Db, password: &str) -> Result<Self> {
        let cipher = Cipher::new_from_password(password);

        Ok(Self { db, cipher })
    }

    pub fn insert<V>(&self, key: &[u8], value: &V) -> Result<()>
    where
        V: Serialize,
    {
        let serialized_value = serde_json::to_vec(value)?;
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
            let deserialized_value = serde_json::from_slice(&decrypted_value)?;
            Ok(Some(deserialized_value))
        } else {
            Ok(None)
        }
    }

    pub fn remove(&self, key: &[u8]) -> Result<Option<sled::IVec>> {
        self.db.remove(key).map_err(Into::into)
    }

    pub fn scan_prefix(&self, prefix: &[u8]) -> impl Iterator<Item = Result<(Vec<u8>, Vec<u8>)>> + '_ {
        self.db.scan_prefix(prefix).map(move |result| {
            match result {
                Ok((key, encrypted_value)) => {
                    match self.cipher.decrypt(&encrypted_value) {
                        Ok(decrypted_value) => Ok((key.to_vec(), decrypted_value)),
                        Err(e) => Err(e.into()),
                    }
                }
                Err(e) => Err(e.into()),
            }
        })
    }
}
