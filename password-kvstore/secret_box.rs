use crate::{Data, Result};
use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, OsRng};
use chacha20poly1305::{ChaCha20Poly1305};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecretBox {
    pub ciphertext: Data,
    pub key: [u8; 32],
    pub nonce: [u8; 12],
}
impl SecretBox {
    pub fn close(data: &[u8]) -> Result<SecretBox> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = Data::new(cipher.encrypt(&nonce, data)?);
        Ok(SecretBox {
            key: key.into(),
            nonce: nonce.into(),
            ciphertext,
        })
    }

    pub fn open(&self) -> Result<Data> {
        let cipher = ChaCha20Poly1305::new((&self.key).into());
        let plaintext = cipher.decrypt((&self.nonce).into(), self.ciphertext.bytes.as_slice())?;
        Ok(Data::new(plaintext))
    }
}

#[test]
fn test_secretbox() -> Result<()> {
    let secret = SecretBox::close(b"secret")?;
    assert_eq!(secret.open()?.to_bytes(), b"secret".to_vec());
    Ok(())
}
