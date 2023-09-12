use std::sync::Arc;

use anyhow::{anyhow, Context};
use tokio::sync::RwLock;

use encryption::Encryption;
use encryption::key::Key;
use storage::{ActiveRecord, FindOneBy};
use storage::db_storage::Database;
use storage::file_storage::FileStorage;
use storage::key_manager::KeyManagerBuilder;
use storage::keyring_storage::KeyringStorage;
use storage::model::password::Password;

pub struct GetPasswordCmd {
    username: String,
    http_address: Option<String>,
    keystore: bool,
}

impl GetPasswordCmd {
    pub fn new(username: String, http_address: Option<String>, keystore: bool) -> Self {
        Self {
            username,
            http_address,
            keystore,
        }
    }

    pub async fn run(&self, db: Arc<RwLock<Database>>) -> anyhow::Result<()> {
        match Password::find_one(db.clone(), FindOneBy::Username(self.username.clone()), None).await? {
            None => Err(anyhow!("{} not found", self.username)),
            Some(password) => {
                let symmetric_key: Key;
                if self.keystore {
                    symmetric_key = self.key_from_keystore()?;
                } else {
                    symmetric_key = self.key_from_file()?;
                }
                let cipher = Encryption::from(symmetric_key);
                let decrypted = cipher.decrypt(
                    &Encryption::to_nonce(password.nonce),
                    password.encrypted_password,
                ).context("Failed to decrypt the password due to the invalid symmetric key")?;

                info!("Password {}", decrypted);
                Ok(())
            }
        }
    }

    fn key_from_file(&self) -> anyhow::Result<Key> {
        let symmetric_key_manager = KeyManagerBuilder::new()
            .with_file_storage(FileStorage::new("symmetric-key.json"))
            .build::<Key>()?;
        symmetric_key_manager.get()
    }

    fn key_from_keystore(&self) -> anyhow::Result<Key> {
        let symmetric_key_manager = KeyManagerBuilder::new()
            .with_keyring_storage(KeyringStorage::new("symmetric-key", "cipher-nest")?)
            .build::<Key>()?;
        symmetric_key_manager.get()
    }
}
