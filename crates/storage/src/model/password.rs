use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use mongodb::bson::{doc, Bson, Document};
use mongodb::error::Error;
use mongodb::options::{
    FindOneOptions, FindOptions, InsertManyOptions, InsertOneOptions, UpdateOptions,
};
use mongodb::Cursor;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::db_storage::Database;
use crate::{ActiveRecord, FindOneBy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    pub username: String,
    pub http_address: Option<String>,
    pub encrypted_password: Vec<u8>,
    pub nonce: Vec<u8>,
}

#[async_trait]
impl ActiveRecord for Password {
    async fn find_one(
        db: Arc<RwLock<Database>>,
        filter: FindOneBy,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Self>, Error>
    where
        Self: Sized + Serialize + for<'de> Deserialize<'de>,
    {
        let read_db = db.read().await;
        match filter {
            FindOneBy::Username(username) => {
                let filter_by_username = doc! { "username": username };
                read_db
                    .password_collection
                    .find_one(filter_by_username, options)
                    .await
            }
            FindOneBy::HttpAddress(http_address) => {
                let filter_by_http_address = doc! { "http_address": http_address };
                read_db
                    .password_collection
                    .find_one(filter_by_http_address, options)
                    .await
            }
        }
    }

    async fn find(
        db: Arc<RwLock<Database>>,
        filter: Document,
        options: Option<FindOptions>,
    ) -> Result<Cursor<Self>, Error>
    where
        Self: Sized + Serialize + for<'de> Deserialize<'de>,
    {
        let read_db = db.read().await;
        read_db.password_collection.find(filter, options).await
    }

    async fn insert(
        &self,
        db: Arc<RwLock<Database>>,
        options: Option<InsertOneOptions>,
    ) -> Result<Bson, Error> {
        let write_db = db.write().await;

        let id = write_db
            .password_collection
            .insert_one(self, options)
            .await
            .expect("failed to insert");

        Ok(id.inserted_id)
    }

    async fn insert_many(
        items: Vec<Self>,
        db: Arc<RwLock<Database>>,
        options: Option<InsertManyOptions>,
    ) -> Result<HashMap<usize, Bson>, Error>
    where
        Self: Sized,
    {
        let write_db = db.write().await;
        let id = write_db
            .password_collection
            .insert_many(items, options)
            .await
            .expect("failed to insert many");

        Ok(id.inserted_ids)
    }

    async fn exists(db: Arc<RwLock<Database>>, filter: FindOneBy) -> Result<bool, Error>
    where
        Self: Sized,
    {
        Self::find_one(db, filter, None)
            .await?
            .map(|_| Ok(true))
            .unwrap_or(Ok(false))
    }

    async fn update(
        db: Arc<RwLock<Database>>,
        filter: Document,
        update: Document,
        options: Option<UpdateOptions>,
    ) -> Result<(), Error>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Password {
    pub fn new(
        username: String,
        http_address: Option<String>,
        encrypted_password: Vec<u8>,
        nonce: Vec<u8>,
    ) -> Self {
        Self {
            username,
            http_address,
            encrypted_password,
            nonce,
        }
    }
}
