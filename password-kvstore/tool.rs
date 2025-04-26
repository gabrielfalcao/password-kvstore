#![allow(unused)]

use argon2_kdf::{Algorithm, Hash, Hasher};
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use chacha20::ChaCha20;
use pbkdf2::pbkdf2_hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sha3::Digest;
pub use sha3::Sha3_384;

use crate::{Data, Error, PlainBytes, Result, Secret};

pub fn hash_password(password: &str) -> Result<Hash> {
    Ok(Hasher::new()
        .algorithm(Algorithm::Argon2id)
        .salt_length(12)
        .hash_length(42)
        .iterations(12)
        .memory_cost_kib(125000)
        .threads(2)
        .hash(password.as_bytes())?)
}

pub struct CipherText {
    pub ciphertext: Data,
    pub nonce: [u8; 12],
}

pub struct Chacha20Tool {
    password: Data,
    iterations: u32,
}

impl Chacha20Tool {
    pub fn new(password: &str, iterations: u32) -> Chacha20Tool {
        Chacha20Tool {
            password: Data::new(password.as_bytes().to_vec()),
            iterations: iterations,
        }
    }

    pub fn hash(&self) -> Result<Hash> {
        Ok(Hasher::new()
            .algorithm(Algorithm::Argon2id)
            .salt_length(12)
            .hash_length(42)
            .iterations(12)
            .memory_cost_kib(125000)
            .threads(2)
            .hash(&self.password.bytes)?)
    }

    pub fn key(&self) -> Result<[u8; 32]> {
        let argon_hash = self.hash()?;
        let mut key = [0; 32];

        let salt = self.password.to_vec();
        let password = bincode::serialize(&self.password)?;
        let mut sha3_384 = Sha3_384::new();
        sha3_384.update(&password);
        let salt = sha3_384.finalize().to_vec();
        pbkdf2_hmac::<Sha256>(&password, &salt, self.iterations, &mut key);
        Ok(key)
    }

    pub fn nonce(&self) -> Result<[u8; 12]> {
        let argon_hash = self.hash()?;
        let mut nonce = [0; 12];
        pbkdf2_hmac::<Sha256>(
            argon_hash.as_bytes(),
            argon_hash.salt_bytes(),
            self.iterations,
            &mut nonce,
        );

        Ok(nonce)
    }

    fn chacha20(&self, nonce: &[u8; 12]) -> Result<ChaCha20> {
        let mut chacha20 = ChaCha20::new(&self.key()?.into(), nonce.into());
        chacha20.seek(self.iterations);
        chacha20.seek(self.password.len());
        Ok(chacha20)
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<CipherText> {
        let key = self.key()?;
        let nonce = self.nonce()?;
        let mut chacha20 = self.chacha20(&nonce)?;
        let mut ciphertext = data.to_vec();
        chacha20.apply_keystream(&mut ciphertext);
        Ok(CipherText {
            ciphertext: Data::new(ciphertext),
            nonce,
        })
    }

    pub fn decrypt(&self, data: &CipherText) -> Result<Vec<u8>> {
        let key = self.key()?;
        let mut chacha20 = self.chacha20(&data.nonce)?;
        let mut plaintext = data.ciphertext.to_vec();
        chacha20.apply_keystream(&mut plaintext);
        Ok(plaintext)
    }
}

#[test]
fn test() -> Result<()> {
    let mut tool = Chacha20Tool::new("password", 600);
    let secret = (0..137)
        .map(|h| format!("{}-secret-{}-", h, h))
        .collect::<String>()
        .as_bytes()
        .to_vec();
    let ciphertext = tool.encrypt(&secret)?;
    let plaintext = tool.decrypt(&ciphertext)?;
    assert_eq!(secret, plaintext);
    Ok(())
}
