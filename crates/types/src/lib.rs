use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub enum StorageFormat {
    FileStorage,
    Keyring,
}

pub trait Storable: Serialize + DeserializeOwned {
    fn stringify(&self, format: StorageFormat) -> Result<String>;
    fn from_str(s: &str, format: StorageFormat) -> Result<Self>
        where
            Self: Sized;
}

pub trait FileStorable {
    fn serialize_for_file(&self) -> Result<String>;
    fn from_file(s: &str) -> Result<Self>
        where
            Self: Sized;
}

pub trait KeyringStorable {
    fn serialize_for_keyring(&self) -> Result<String>;
    fn from_keyring_str(s: &str) -> Result<Self>
        where
            Self: Sized;
}

impl<T: FileStorable + KeyringStorable + serde::Serialize + for<'de> serde::Deserialize<'de>>
Storable for T
{
    fn stringify(&self, format: StorageFormat) -> Result<String> {
        match format {
            StorageFormat::FileStorage => self.serialize_for_file(),
            StorageFormat::Keyring => self.serialize_for_keyring(),
        }
    }

    fn from_str(s: &str, format: StorageFormat) -> Result<Self>
        where
            Self: Sized,
    {
        match format {
            StorageFormat::FileStorage => Ok(Self::from_file(s).map_err(|e| anyhow!(e))?),
            StorageFormat::Keyring => Ok(Self::from_keyring_str(s).map_err(|e| anyhow!(e))?),
        }
    }
}
