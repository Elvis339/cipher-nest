#![feature(async_fn_in_trait)]

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use mongodb::bson::Bson;
use mongodb::Cursor;
use mongodb::options::{
    FindOneOptions, FindOptions, InsertManyOptions, InsertOneOptions, UpdateOptions,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use types::Storable;

use crate::db_storage::Database;

pub mod db_storage;
pub mod file_storage;
pub mod key_manager;
pub mod keyring_storage;
pub mod model;

#[async_trait]
pub trait ActiveRecord<Filter>
    where
        Filter: Serialize + for<'de> Deserialize<'de>,
{
    async fn find_one(
        db: Arc<RwLock<Database>>,
        filter: Filter,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Self>, mongodb::error::Error>
        where
            Self: Sized + Serialize + for<'de> Deserialize<'de>;

    async fn find(
        db: Arc<RwLock<Database>>,
        filter: Filter,
        options: Option<FindOptions>,
    ) -> Result<Cursor<Self>, mongodb::error::Error>
        where
            Self: Sized + Serialize + for<'de> Deserialize<'de>;

    async fn insert(
        &self,
        db: Arc<RwLock<Database>>,
        options: Option<InsertOneOptions>,
    ) -> Result<Bson, mongodb::error::Error>;

    async fn insert_many(
        items: Vec<Self>,
        db: Arc<RwLock<Database>>,
        options: Option<InsertManyOptions>,
    ) -> Result<HashMap<usize, Bson>, mongodb::error::Error>
        where
            Self: Sized;

    async fn exists(
        db: Arc<RwLock<Database>>,
        filter: Filter,
    ) -> Result<bool, mongodb::error::Error>
        where
            Self: Sized;

    //
    // async fn update(
    //     db: Arc<RwLock<Database>>,
    //     filter: Filter,
    //     update: mongodb::bson::Document,
    //     options: Option<UpdateOptions>,
    // ) -> Result<(), mongodb::error::Error>
    // where
    //     Self: Sized;
}

pub trait Storage<T: Storable> {
    fn save(&self, item: T) -> anyhow::Result<()>;
    fn get(&self) -> anyhow::Result<T>;
}
