# Cipher Nest Password Manager

Welcome to the Cipher Nest Password Manager project! 
If you are someone who prefers taking security into their own hands and are weary of paying for password managers that are susceptible to breaches, feel free to use and modify this free open-source solution.

## How to Use

At present, the primary goal of this project is to support synchronization across devices. 
Currently, interaction with the software is possible through a command-line interface (CLI). 
Future plans include the development of a browser plugin and a mobile application, enhancing accessibility and user experience.

> Note: The Keychain functionality is currently only supported on macOS.

### Key Management

The management of keys is a critical aspect of this application. 
By default, the symmetric key and salt utilized for key generation are saved in the `~/.cipher-nest` directory, unless the `--keystore` flag is specified, in which case it will be stored in the Keychain. 
The key derivation process involves the use of a master password (set by the user) and a salt (randomly generated if not specified) utilizing the PBKDF2 construct.

#### Why PBKDF2?

PBKDF2 enhances data security by generating a cryptographic key from the user's master password and a salt, protecting against brute-force and rainbow table attacks. 
Importantly, the salt can be shared without revealing the master key, facilitating secure synchronization across various devices. 
This addition of salt increases encryption resilience, offering a robust password management solution without compromising the master key's safety.

### Encryption Scheme

For the encryption, the robust and secure ChaCha20Poly1305 scheme is employed.

## Getting Started

### Requirements
- Rust nightly 1.72
- Mongodb in a docker

1. Start mongodb: `docker run -d -p 27017:27017 --name=cipher-nest mongo:latest`
2. Build the project: `cargo build`
3. Run the CLI: `./target/debug/client --help`

## Contribution

If you'd like to contribute to the project, please feel free to open a pull request. Your contributions are most welcome.