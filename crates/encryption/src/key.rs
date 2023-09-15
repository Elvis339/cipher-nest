use anyhow::{Context, Result};
use pbkdf2::pbkdf2_hmac_array;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use types::{FileStorable, KeyringStorable};

use crate::salt::Salt;

/// Encryption and Decryption key
#[derive(Serialize, Deserialize, Clone)]
pub struct Key {
    inner: Vec<u8>,
}

impl FileStorable for Key {
    fn serialize_for_file(&self) -> Result<String> {
        serde_json::to_string(&self).context("failed to serialize key")
    }

    fn from_file(s: &str) -> Result<Self>
        where
            Self: Sized,
    {
        let key: Key = serde_json::from_str(s)?;
        Ok(key)
    }
}

impl KeyringStorable for Key {
    fn serialize_for_keyring(&self) -> Result<String> {
        Ok(hex::encode(&self.inner))
    }

    fn from_keyring_str(s: &str) -> Result<Self>
        where
            Self: Sized,
    {
        let key = hex::decode(s).context("failed to deserialize key from keyring")?;

        Ok(Key { inner: key })
    }
}

impl Key {
    pub fn new(master_password: String, salt: Salt) -> Self {
        let hashing_rounds = 50_000;
        Self {
            inner: pbkdf2_hmac_array::<Sha256, 32>(
                master_password.as_bytes(),
                salt.as_slice(),
                hashing_rounds,
            )
                .to_vec(),
        }
    }

    pub fn get_key(&self) -> &Vec<u8> {
        &self.inner
    }
}
