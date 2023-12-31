#![feature(fs_mode)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use anyhow::Context;

use types::{Storable, StorageFormat};

use crate::Storage;

pub struct FileStorage {
    file_path: PathBuf,
}

impl<T: Storable> Storage<T> for FileStorage {
    fn save(&self, item: T) -> anyhow::Result<()> {
        self.ensure_directory_exists()?;

        let data = item.stringify(StorageFormat::FileStorage)?;
        fs::write(&self.file_path, data).context("Failed to write data to file")?;
        Ok(())
    }

    fn get(&self) -> anyhow::Result<T> {
        let data = fs::read_to_string(&self.file_path).context("Symmetric key not found")?;
        T::from_str(&data, StorageFormat::FileStorage)
            .context("Failed to deserialize item from JSON")
    }
}

impl FileStorage {
    pub fn new(file_name: &str) -> Self {
        let cipher_nest_dir = Self::get_cipher_nest_dir();
        let file_path = cipher_nest_dir.join(file_name);

        Self { file_path }
    }

    fn ensure_directory_exists(&self) -> anyhow::Result<()> {
        if let Some(dir) = self.file_path.parent() {
            if !dir.exists() {
                fs::create_dir_all(dir).context("Failed to create .cipher-nest directory")?;

                // Set permissions to allow only owner read and write access
                let permissions = fs::Permissions::from_mode(0o700);
                fs::set_permissions(dir, permissions)
                    .context("Failed to set directory permissions")?;
            }
        }
        Ok(())
    }

    pub fn get_cipher_nest_dir() -> PathBuf {
        let home_dir = dirs::home_dir().expect("Unable to get home directory");
        let cipher_nest_dir = home_dir.join(".cipher-nest");

        cipher_nest_dir
    }
}
