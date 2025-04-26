use argon2_kdf::{Algorithm, Hasher};
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use chacha20::ChaCha20;
use pbkdf2::pbkdf2_hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sha3::{Digest, Sha3_384};

use crate::{Data, Result, SecretBox};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CipherText {
    ciphertext: Data,
    nonce: [u8; 12],
}
impl CipherText {
    pub fn new(ciphertext: Data, nonce: [u8;12]) -> CipherText {
        CipherText {
            ciphertext, nonce
        }

    }
    pub fn ciphertext(&self) -> Data {
        self.ciphertext.clone()
    }
    pub fn nonce(&self) -> [u8;12] {
        self.nonce.clone()
    }
}

pub struct Chacha20Tool {
    password: SecretBox,
    iterations: u32,
}

impl Chacha20Tool {
    pub fn new(password: &str, iterations: u32) -> Result<Chacha20Tool> {
        Ok(Chacha20Tool {
            password: SecretBox::close(&password.as_bytes())?,
            iterations: iterations,
        })
    }

    pub fn hash(&self) -> Result<(Vec<u8>, Vec<u8>)> {
        let bytes = self.password.open()?.to_bytes();
        let hash = Hasher::new()
            .algorithm(Algorithm::Argon2id)
            .salt_length(12)
            .hash_length(42)
            .iterations(12)
            .memory_cost_kib(125000)
            .threads(2)
            .hash(&bytes)?;
        Ok((hash.as_bytes().to_vec(), hash.salt_bytes().to_vec()))
    }

    pub fn key(&self) -> Result<[u8; 32]> {
        let mut key = [0; 32];
        let password = bincode::serialize(&self.password.open()?.bytes)?;
        let mut sha3_384 = Sha3_384::new();
        sha3_384.update(&password);
        let salt = sha3_384.finalize().to_vec();
        pbkdf2_hmac::<Sha256>(&password, &salt, self.iterations, &mut key);
        Ok(key)
    }

    pub fn nonce(&self) -> Result<[u8; 12]> {
        let (hash_bytes, salt_bytes) = self.hash()?;
        let mut nonce = [0; 12];
        pbkdf2_hmac::<Sha256>(
            &hash_bytes,
            &salt_bytes,
            self.iterations,
            &mut nonce,
        );

        Ok(nonce)
    }

    fn chacha20(&self, nonce: &[u8; 12]) -> Result<ChaCha20> {
        let mut chacha20 = ChaCha20::new(&self.key()?.into(), nonce.into());
        chacha20.seek(self.iterations);
        chacha20.seek(self.password.open()?.len());
        Ok(chacha20)
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<CipherText> {
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
        let mut chacha20 = self.chacha20(&data.nonce)?;
        let mut plaintext = data.ciphertext.to_vec();
        chacha20.apply_keystream(&mut plaintext);
        Ok(plaintext)
    }
}

#[test]
fn test_chacha20tool() -> Result<()> {
    let tool = Chacha20Tool::new("password", 600)?;
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
