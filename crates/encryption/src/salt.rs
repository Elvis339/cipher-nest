use anyhow::Context;
use rand::Rng;
use serde::{Deserialize, Serialize};

use types::{FileStorable, KeyringStorable};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Salt {
    inner: Vec<u8>,
}

impl FileStorable for Salt {
    fn serialize_for_file(&self) -> anyhow::Result<String> {
        serde_json::to_string(&self).context("failed to serialize salt for file")
    }

    fn from_file(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        serde_json::from_str(s).context("failed to deserialize salt from file")
    }
}

impl KeyringStorable for Salt {
    fn serialize_for_keyring(&self) -> anyhow::Result<String> {
        serde_json::to_string(&self.inner).context("failed to serialize salt for file")
    }

    fn from_keyring_str(s: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Salt {
            inner: s.as_bytes().to_vec(),
        })
    }
}

impl Salt {
    pub fn new(salt: Vec<u8>) -> Self {
        Self { inner: salt }
    }

    pub fn generate_random() -> Self {
        let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
        Self {
            inner: random_bytes.to_vec(),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }
}
