#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::sync::Arc;

use clap::{Parser, Subcommand};
use clap::builder::Str;
use tokio::sync::RwLock;

use storage::db_storage::Database;

use crate::cmd::get_password_cmd::GetPasswordCmd;
use crate::cmd::new_key_cmd::NewKeyCmd;
use crate::cmd::save_password_cmd::SavePasswordCmd;

mod cmd;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "New key")]
    Key {
        #[arg(short, long)]
        master_password: String,

        #[arg(short, long, required = false)]
        salt: Option<String>,

        #[arg(
        short,
        long,
        action,
        required = false,
        help = "Save to keystore, by default it will save to file"
        )]
        keystore: bool,
    },
    #[command(
    about = "Encrypted password unless specified otherwise symmetric key will be derived from the file"
    )]
    Save {
        #[arg(short, long, help = "Unique identifier usually an email")]
        username: String,

        #[arg(short = 'a', long, help = "Associate website with the password")]
        http_address: Option<String>,

        #[arg(short, long, help = "Password to save")]
        password: String,

        #[arg(
            short,
            long,
            action,
            required = false,
            help = "Get the symmetric key from keystore"
        )]
        keystore: bool,
    },
    #[command(about = "Decrypted password")]
    Get {
        #[arg(short, long, help = "By username")]
        username: String,

        #[arg(short = 'a', long, help = "By http_address", required = false)]
        http_address: Option<String>,

        #[arg(
            short,
            long,
            action,
            required = false,
            help = "Get the symmetric key from keystore"
        )]
        keystore: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let args = Cli::parse();

    let db = Arc::new(RwLock::new(
        Database::new().await.expect("failed to start database"),
    ));

    match args.command {
        Commands::Key {
            master_password,
            salt,
            keystore,
        } => NewKeyCmd::new(master_password, salt, keystore)
            .run()
            .unwrap(),
        Commands::Save {
            username,
            http_address,
            password,
            keystore,
        } => {
            SavePasswordCmd::new(username, http_address, password, keystore)
                .run(db.clone())
                .await?
        }
        Commands::Get {
            username,
            http_address,
            keystore,
        } => {
            GetPasswordCmd::new(username, http_address, keystore)
                .run(db.clone())
                .await?
        }
    }

    Ok(())
}
