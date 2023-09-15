use std::sync::Arc;

use anyhow::bail;
use curve25519_dalek::RistrettoPoint;
use tokio::sync::RwLock;

use storage::db_storage::Database;
use storage::model::tenant::{Tenant, TenantFilter};
use storage::ActiveRecord;
use zkp::ecc_chaum_pedersen::EccChaumPedersen;
use zkp::ChaumPedersenTrait;

pub struct AuthenticateCommand {
    username: String,
    master_password: String,
}

impl AuthenticateCommand {
    pub fn new(username: String, master_password: String) -> Self {
        Self {
            username,
            master_password,
        }
    }

    pub async fn run(&self, db: Arc<RwLock<Database>>) -> anyhow::Result<()> {
        if !Tenant::exists(
            db.clone(),
            TenantFilter {
                username: self.username.clone(),
            },
        )
        .await?
        {
            bail!("tenant not found")
        }

        let secret_str = self.master_password.clone();
        let secret_bytes = secret_str.as_bytes();
        let secret_x = EccChaumPedersen::hash(secret_bytes);

        let schema = EccChaumPedersen::new();
        let (k, challenge, _) = schema.prover_commit().await;
        let solution = schema.prover_solve_challenge(k, challenge.unwrap(), secret_x);

        let tenant = Tenant::find_one(
            db.clone(),
            TenantFilter {
                username: self.username.clone(),
            },
            None,
        )
        .await?
        .unwrap();
        let y1: RistrettoPoint = serde_json::from_str(&tenant.y1).unwrap();
        let y2: RistrettoPoint = serde_json::from_str(&tenant.y2).unwrap();

        if !schema
            .verify_proof(solution, challenge.unwrap(), y1, y2, None, None)
            .await
        {
            bail!("invalid proof")
        }

        Ok(())
    }
}
