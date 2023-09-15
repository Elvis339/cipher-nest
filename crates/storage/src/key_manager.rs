use anyhow::bail;

use types::Storable;

use crate::file_storage::FileStorage;
use crate::keyring_storage::KeyringStorage;
use crate::Storage;

pub struct KeyManager<T: Storable> {
    storage: Box<dyn Storage<T>>,
}

impl<T: Storable> KeyManager<T> {
    fn new<S: Storage<T> + 'static>(storage: S) -> Self {
        Self {
            storage: Box::new(storage),
        }
    }

    pub fn save(&self, item: T) -> anyhow::Result<()> {
        self.storage.save(item)
    }

    pub fn get(&self) -> anyhow::Result<T> {
        self.storage.get()
    }
}

pub struct KeyManagerBuilder {
    file_storage: Option<FileStorage>,
    keyring_storage: Option<KeyringStorage>,
}

impl KeyManagerBuilder {
    pub fn new() -> Self {
        Self {
            file_storage: None,
            keyring_storage: None,
        }
    }

    pub fn with_file_storage(mut self, file_storage: FileStorage) -> Self {
        self.file_storage = Some(file_storage);
        self
    }

    pub fn with_keyring_storage(mut self, keyring_storage: KeyringStorage) -> Self {
        self.keyring_storage = Some(keyring_storage);
        self
    }

    pub fn build<T: Storable>(self) -> anyhow::Result<KeyManager<T>> {
        match (self.file_storage, self.keyring_storage) {
            (Some(fs), None) => Ok(KeyManager::new(fs)),
            (None, Some(ks)) => Ok(KeyManager::new(ks)),
            _ => bail!("Either file storage or keyring storage must be set, but not both."),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Context;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use types::{FileStorable, KeyringStorable};

    use super::*;

    /// Encryption and Decryption key
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Key {
        inner: Vec<u8>,
    }

    impl FileStorable for Key {
        fn serialize_for_file(&self) -> anyhow::Result<String> {
            serde_json::to_string(&self).context("failed to serialize")
        }

        fn from_file(s: &str) -> anyhow::Result<Self>
            where
                Self: Sized,
        {
            let item: Key = serde_json::from_str(s).context("failed to deserialize")?;

            Ok(item)
        }
    }

    impl KeyringStorable for Key {
        fn serialize_for_keyring(&self) -> anyhow::Result<String> {
            Ok(hex::encode(&self.inner))
        }

        fn from_keyring_str(s: &str) -> anyhow::Result<Self>
            where
                Self: Sized,
        {
            let key = hex::decode(s).context("failed to deserialize key from keyring")?;

            Ok(Key { inner: key })
        }
    }

    fn new_key() -> (String, String, Key) {
        let key = Key {
            inner: vec![
                80, 12, 189, 125, 177, 174, 127, 180, 107, 234, 166, 143, 49, 162, 104, 198, 43,
                14, 199, 190, 183, 164, 241, 13, 181, 153, 205, 255, 40, 134, 21, 120,
            ],
        };

        ("password1".to_string(), "salt1".to_string(), key)
    }

    fn cleanup(path: &str) {
        let dir = FileStorage::get_cipher_nest_dir();
        let file = dir.join(path);

        fs::remove_file(file).unwrap()
    }

    #[test]
    fn file_storage() {
        let fs = FileStorage::new("test_master_password");
        let key_manager = KeyManagerBuilder::new()
            .with_file_storage(fs)
            .build::<Key>()
            .unwrap();

        let (password, salt, key) = new_key();
        key_manager.save(key.clone()).unwrap();

        assert!(key_manager.save(key.clone()).is_ok());
        cleanup("test_master_password");
    }

    #[test]
    fn keyring_storage() {
        let ks = KeyringStorage::new("test", "test").unwrap();
        let key_manager = KeyManagerBuilder::new()
            .with_keyring_storage(ks.clone())
            .build::<Key>()
            .unwrap();

        let (password, salt, key) = new_key();
        assert!(key_manager.save(key.clone()).is_ok());
        ks.delete().unwrap();
    }
}
