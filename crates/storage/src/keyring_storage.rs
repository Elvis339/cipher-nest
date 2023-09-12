use anyhow::{anyhow, Context};
use keyring::Entry;
use serde_json::json;

use types::{Storable, StorageFormat};

use crate::Storage;

pub struct KeyringStorage {
    service: String,
    name: String,
    entry: Entry,
}

impl Clone for KeyringStorage {
    fn clone(&self) -> Self {
        let service = &self.service;
        let name = &self.name;

        Self {
            service: service.clone(),
            name: name.clone(),
            entry: Entry::new(service, name).unwrap(),
        }
    }
}

impl<T: Storable> Storage<T> for KeyringStorage {
    fn save(&self, item: T) -> anyhow::Result<()> {
        let data = item.stringify(StorageFormat::Keyring)?;
        self.entry.set_password(data.as_str())?;
        Ok(())
    }

    fn get(&self) -> anyhow::Result<T> {
        let data = self.entry.get_password()?;
        T::from_str(&data, StorageFormat::Keyring).context("Failed to deserialize item from JSON")
    }
}

impl KeyringStorage {
    pub fn new(service: &str, name: &str) -> anyhow::Result<Self> {
        let entry = Entry::new(service, name).map_err(|e| anyhow!(e))?;
        Ok(Self {
            entry,
            service: String::from(service),
            name: String::from(name),
        })
    }

    pub fn delete(&self) -> anyhow::Result<()> {
        self.entry.delete_password()?;

        Ok(())
    }

    pub fn get_service_name(&self) -> &String {
        &self.service
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
