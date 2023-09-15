use std::sync::Arc;

use anyhow::bail;
use tokio::sync::RwLock;

use storage::ActiveRecord;
use storage::db_storage::Database;
use storage::model::tenant::{Tenant, TenantFilter};
use zkp::ChaumPedersenTrait;
use zkp::ecc_chaum_pedersen::EccChaumPedersen;

pub struct RegisterCommand {
    username: String,
    master_password: String,
}

impl RegisterCommand {
    pub fn new(username: String, master_password: String) -> Self {
        Self {
            username,
            master_password,
        }
    }

    pub async fn run(&self, db: Arc<RwLock<Database>>) -> anyhow::Result<()> {
        if Tenant::exists(
            db.clone(),
            TenantFilter {
                username: self.username.clone(),
            },
        )
            .await?
        {
            bail!("tenant with username of {} already exist", self.username)
        }

        let schema = EccChaumPedersen::new();
        let (y1, y2) = schema
            .generate_public_keys(EccChaumPedersen::hash(self.master_password.as_bytes()))
            .await;

        let y1_ser = serde_json::to_string(&y1).unwrap();
        let y2_ser = serde_json::to_string(&y2).unwrap();

        let tenant = Tenant::new(self.username.clone(), y1_ser, y2_ser);
        tenant.insert(db.clone(), None).await?;

        Ok(())
    }
}
