use anyhow::Context;

use encryption::key::Key;
use encryption::salt::Salt;
use storage::file_storage::FileStorage;
use storage::key_manager::KeyManagerBuilder;
use storage::keyring_storage::KeyringStorage;

pub struct NewKeyCmd {
    master_password: String,
    salt: Salt,
    keystore: bool,
}

impl NewKeyCmd {
    pub fn new(master_password: String, salt: Option<String>, keystore: bool) -> Self {
        let salt_value = salt
            .map(|s| Salt::new(s.as_bytes().to_vec()))
            .unwrap_or(Salt::generate_random());
        Self {
            master_password,
            salt: salt_value,
            keystore,
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let symmetric_key = Key::new(self.master_password.clone(), self.salt.clone());

        if self.keystore {
            self.save_to_keyring(symmetric_key)?;
        } else {
            self.save_to_file(symmetric_key)?;
        }

        self.save_salt_to_file()
    }

    fn save_to_keyring(&self, key: Key) -> anyhow::Result<()> {
        let keyring_storage = KeyringStorage::new("symmetric-key", "cipher-nest")
            .context("Failed to instantiate keyring storage for symmetric-key")?;

        let mut key_manager_builder = KeyManagerBuilder::new();
        let key_manager = key_manager_builder
            .with_keyring_storage(keyring_storage)
            .build::<Key>()?;
        key_manager.save(key)?;

        Ok(())
    }

    fn save_to_file(&self, key: Key) -> anyhow::Result<()> {
        let file_storage = FileStorage::new("symmetric-key.json");

        let mut key_manager_builder = KeyManagerBuilder::new();
        let key_manager = key_manager_builder
            .with_file_storage(file_storage)
            .build::<Key>()?;
        key_manager.save(key)?;

        Ok(())
    }

    fn save_salt_to_file(&self) -> anyhow::Result<()> {
        let file_storage = FileStorage::new("salt.json");

        let mut key_manager_builder = KeyManagerBuilder::new();
        let salt_key_manager = key_manager_builder
            .with_file_storage(file_storage)
            .build::<Salt>()?;
        salt_key_manager.save(self.salt.clone())?;

        info!(
            "Salt saved at {}",
            FileStorage::get_cipher_nest_dir().to_str().unwrap()
        );

        Ok(())
    }
}
