use clap::{Args, Parser, Subcommand, ValueEnum};
use encryption::key::Key;
use encryption::salt::Salt;
use encryption::Encryption;
use storage::file_storage::FileStorage;
use storage::key_manager::KeyManagerBuilder;
use storage::keyring_storage::KeyringStorage;

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
    #[command(about = "Encrypted password")]
    Save {
        #[arg(short = 'p', long)]
        password: String,
    },
    #[command(about = "Decrypted password")]
    Get {
        #[arg(
            long,
            require_equals = true,
            value_name = "FILTER",
            num_args = 0..=1,
            default_value_t = GetByFilter::Username,
            default_missing_value = "username",
            value_enum
        )]
        filter: GetByFilter,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum GetByFilter {
    Username,
    HttpAddress,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Key {
            master_password,
            salt,
            keystore,
        } => {
            let salt_value = salt
                .map(|s| Salt::new(s.as_bytes().to_vec()))
                .unwrap_or(Salt::generate_random());

            let symmetric_key = Key::new(master_password, salt_value.clone());
            let mut key_manager_builder = KeyManagerBuilder::new();

            if keystore {
                let _keyring_key_manager = key_manager_builder
                    .with_keyring_storage(
                        KeyringStorage::new("cipher-nest", "symmetric-key").unwrap(),
                    )
                    .build::<Key>()
                    .unwrap();
            } else {
                let symmetric_manager = key_manager_builder
                    .with_file_storage(FileStorage::new("symmetric-key.json"))
                    .build::<Key>()
                    .unwrap();

                let ff = KeyManagerBuilder::new();
                let salt_manager = ff
                    .with_file_storage(FileStorage::new("salt.json"))
                    .build::<Salt>()
                    .unwrap();

                symmetric_manager.save(symmetric_key).unwrap();
                salt_manager.save(salt_value).unwrap();
                println!(
                    "save to {}",
                    FileStorage::get_cipher_nest_dir()
                        .join("symmetric-key.json")
                        .to_str()
                        .unwrap()
                );
            }
        }
        Commands::Save { password } => {}
        Commands::Get { filter } => {
            println!("{:?}", filter)
        }
    }
}
