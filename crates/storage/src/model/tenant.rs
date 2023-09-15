use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
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
use crate::ActiveRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub username: String,
    pub y1: String,
    pub y2: String,
}

#[derive(Serialize, Deserialize)]
pub struct TenantFilter {
    pub username: String,
}

#[async_trait]
impl ActiveRecord<TenantFilter> for Tenant {
    async fn find_one(
        db: Arc<RwLock<Database>>,
        filter: TenantFilter,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Self>, Error>
    where
        Self: Sized + Serialize + for<'de> Deserialize<'de>,
    {
        let read_db = db.read().await;
        let username_filter = doc! { "username": filter.username };
        read_db
            .tenant_collection
            .find_one(username_filter, options)
            .await
    }

    async fn find(
        _db: Arc<RwLock<Database>>,
        _filter: TenantFilter,
        _options: Option<FindOptions>,
    ) -> Result<Cursor<Self>, Error>
    where
        Self: Sized + Serialize + for<'de> Deserialize<'de>,
    {
        unimplemented!()
    }

    async fn insert(
        &self,
        db: Arc<RwLock<Database>>,
        options: Option<InsertOneOptions>,
    ) -> Result<Bson, Error> {
        let write_db = db.write().await;
        let result = write_db.tenant_collection.insert_one(self, options).await?;
        Ok(result.inserted_id)
    }

    async fn insert_many(
        items: Vec<Self>,
        db: Arc<RwLock<Database>>,
        options: Option<InsertManyOptions>,
    ) -> Result<HashMap<usize, Bson>, Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    async fn exists(db: Arc<RwLock<Database>>, filter: TenantFilter) -> Result<bool, Error>
    where
        Self: Sized,
    {
        let item = Self::find_one(db, filter, None).await?;
        return Ok(item.is_some());
    }

    // async fn update(
    //     db: Arc<RwLock<Database>>,
    //     filter: Document,
    //     update: Document,
    //     options: Option<UpdateOptions>,
    // ) -> Result<(), Error>
    // where
    //     Self: Sized,
    // {
    //     todo!()
    // }
}

impl Tenant {
    pub fn new(username: String, y1: String, y2: String) -> Self {
        Self { username, y1, y2 }
    }
}
