#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::sync::Arc;

use clap::{Parser, Subcommand};
use tokio::sync::RwLock;

use storage::db_storage::Database;

use crate::cmd::authenticate_cmd::AuthenticateCommand;
use crate::cmd::get_password_cmd::GetPasswordCmd;
use crate::cmd::new_key_cmd::NewKeyCmd;
use crate::cmd::register_cmd::RegisterCommand;
use crate::cmd::save_password_cmd::SavePasswordCmd;

mod cmd;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Test,
    Register {
        #[arg(short, long, help = "Unique username")]
        username: String,

        #[arg(
            short,
            long,
            help = "Could be different then the master password used to derive symmetric key"
        )]
        master_password: String,
    },

    #[command(
        about = "Prove you are who you claim you are, the password is not stored on the server."
    )]
    Authenticate {
        #[arg(short, long, help = "Username")]
        username: String,

        #[arg(
            short,
            long,
            help = "Could be different then the master password used to derive symmetric key"
        )]
        master_password: String,
    },

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
        Commands::Test => {
            let db_cloned = db.clone();
            let write = db_cloned.write().await;
            let tt = write.create_tenant().await;
            println!("tt = {:?}", tt);
        }
        Commands::Register {
            username,
            master_password,
        } => {
            RegisterCommand::new(username, master_password)
                .run(db.clone())
                .await?
        }
        Commands::Authenticate { username, master_password } => {
            AuthenticateCommand::new(username, master_password)
                .run(db.clone())
                .await?
        }
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
