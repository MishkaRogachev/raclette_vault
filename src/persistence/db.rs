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

    pub fn upsert<V>(&self, key: &[u8], value: &V, encrypted: bool) -> Result<()>
    where
        V: Serialize,
    {
        let mut value = serde_json::to_vec(value)?;
        if encrypted {
            value = self.cipher.encrypt(&value)?;
        }
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn get<V>(&self, key: &[u8], encrypted: bool) -> Result<Option<V>>
    where
        V: for<'de> Deserialize<'de>,
    {
        if let Some(mut value) = self.db.get(key)? {
            if encrypted {
                value = self.cipher.decrypt(&value)?.into();
            }
            Ok(Some(serde_json::from_slice(&value)?))
        } else {
            Ok(None)
        }
    }

    pub fn remove(&self, key: &[u8]) -> Result<Option<sled::IVec>> {
        self.db.remove(key).map_err(Into::into)
    }

    pub fn scan_prefix<V>(&self, prefix: &[u8], cursor: usize, count: usize, encrypted: bool) -> Result<Vec<V>>
    where V: for<'de> Deserialize<'de> {
        self.db.scan_prefix(prefix)
            .skip(cursor).take(count)
            .map(move |result| {
            match result {
                Ok((_key, mut value)) => {
                    if encrypted {
                        value = self.cipher.decrypt(&value)?.into();
                    }
                    Ok(serde_json::from_slice(&value)?)
                }
                Err(e) => Err(e.into()),
            }
        }).collect()
    }
}
