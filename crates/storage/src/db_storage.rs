use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client, Collection, Database as MongoDB};

use crate::model::password::Password;
use crate::model::tenant::Tenant;

pub struct Database {
    client: Client,
    db: MongoDB,
    pub password_collection: Collection<Password>,
    pub tenant_collection: Collection<Tenant>,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("password_manager".to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database("cipher-nest");

        let tenant_collection = database.collection::<Tenant>("tenants");
        let password_collection = database.collection::<Password>("passwords");

        Ok(Self {
            client,
            db: database,
            tenant_collection,
            password_collection,
        })
    }
}
