use mongodb::{options::ClientOptions, Client, Collection};

use crate::model::password::Password;

pub struct Database {
    client: Client,
    pub password_collection: Collection<Password>,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("password_manager".to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database("cipher-nest");

        let password_collection = database.collection::<Password>("passwords");

        Ok(Self {
            client,
            password_collection,
        })
    }
}
