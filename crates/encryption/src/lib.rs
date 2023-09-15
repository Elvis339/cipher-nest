extern crate core;

use anyhow::anyhow;
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, KeyInit};
use chacha20poly1305::{Key as KeyChaChaPoly1305, Nonce};
use chacha20poly1305::aead::{Aead, Result as AeadResult};
use rand_core::OsRng;

use crate::key::Key;

pub mod key;
pub mod salt;

pub struct Encryption {
    cipher: ChaCha20Poly1305,
}

impl Encryption {
    pub fn encrypt(&self, nonce: &Nonce, msg: &[u8]) -> AeadResult<Vec<u8>> {
        self.cipher.encrypt(&nonce, msg.as_ref())
    }

    pub fn decrypt(&self, nonce: &Nonce, ciphertext: Vec<u8>) -> anyhow::Result<String> {
        let decrypted_bytes = self
            .cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| anyhow!(e))?;
        let password = String::from_utf8(decrypted_bytes).map_err(|e| anyhow!(e))?;
        Ok(password)
    }

    pub fn generate_nonce() -> Nonce {
        ChaCha20Poly1305::generate_nonce(&mut OsRng)
    }

    pub fn to_nonce(nonce: Vec<u8>) -> Nonce {
        Nonce::clone_from_slice(nonce.as_slice())
    }
}

impl From<Key> for Encryption {
    fn from(value: Key) -> Self {
        let binding = value.get_key().as_slice();
        let symmetric_key = KeyChaChaPoly1305::from_slice(binding);
        let cipher = ChaCha20Poly1305::new(symmetric_key);
        Self { cipher }
    }
}

#[cfg(test)]
mod tests {
    use crate::salt::Salt;

    use super::*;

    #[test]
    fn encryption_decryption() {
        let symmetric_key = Key::new("password1".to_string(), Salt::new(b"salt1".to_vec()));
        let encryption = Encryption::from(symmetric_key);
        let nonce = Encryption::generate_nonce();

        let msg = "hello world";

        let ciphertext = encryption.encrypt(&nonce, msg.as_bytes()).unwrap();
        assert_eq!(
            msg.to_string(),
            encryption.decrypt(&nonce, ciphertext).unwrap()
        );
    }
}
