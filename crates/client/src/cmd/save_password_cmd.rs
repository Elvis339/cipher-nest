use std::sync::Arc;

use tokio::sync::RwLock;

use encryption::Encryption;
use encryption::key::Key;
use storage::ActiveRecord;
use storage::db_storage::Database;
use storage::file_storage::FileStorage;
use storage::key_manager::KeyManagerBuilder;
use storage::keyring_storage::KeyringStorage;
use storage::model::password::Password;

pub struct SavePasswordCmd {
    username: String,
    http_address: Option<String>,
    password: String,
    keystore: bool,
}

impl SavePasswordCmd {
    pub fn new(
        username: String,
        http_address: Option<String>,
        password: String,
        keystore: bool,
    ) -> Self {
        Self {
            username,
            http_address,
            password,
            keystore,
        }
    }

    pub async fn run(&self, db: Arc<RwLock<Database>>) -> anyhow::Result<()> {
        let symmetric_key: Key;
        if self.keystore {
            symmetric_key = self.key_from_keystore()?;
        } else {
            symmetric_key = self.key_from_file()?;
        }

        let cipher = Encryption::from(symmetric_key);
        let nonce = Encryption::generate_nonce();
        let encrypted_password = cipher.encrypt(&nonce, self.password.as_bytes()).unwrap();

        {
            let password = Password::new(
                self.username.clone(),
                self.http_address.clone(),
                encrypted_password,
                nonce.to_vec(),
            );

            let record_id = password.insert(db, None).await?;
            info!("Successfully created new record {}", record_id);
        }

        Ok(())
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
