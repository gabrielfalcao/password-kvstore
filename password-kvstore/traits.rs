// use std::iter::Iterator;

use serde::{Deserialize, Serialize};

// use crate::{Data, DataSeq};
use crate::{Result};

pub trait PlainBytes: for<'a> Deserialize<'a> + Serialize + Sized {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_plain_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_plain_bytes(bytes)
            .expect(&format!("{}::from_plain_bytes", std::any::type_name::<Self>()))
    }
    fn to_plain_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("bytes")
    }
    fn from_plain_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(&bytes)?)
    }
    fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        crate::to_flate_bytes(self)
    }
    fn from_deflate_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(crate::from_deflate_bytes::<Self>(bytes)?)
    }
    fn to_hex(&self) -> String {
        sanitation::to_hex(&self.to_bytes())
    }
}


// pub trait EncryptionKey: Clone {
//     fn encrypt(&self, data: impl Iterator<Item = u8>) -> Result<Data> {
//         let data_seq = self.encrypt_bytes(&data.collect::<Vec<u8>>())?;
//         data_seq.to_data()
//     }
//     fn encrypt_bytes(&self, data: &[u8]) -> Result<DataSeq>;
// }

// pub trait DecryptionKey: Clone {
//     fn decrypt(&self, data: impl Iterator<Item = u8>) -> Result<Data> {
//         let enc_seq = DataSeq::from_data(&Data::new(data.collect::<Vec<u8>>()))?;
//         let dec_sec = self.decrypt_bytes(enc_seq)?;
//         let mut data = Data::new(Vec::new());
//         for chunk in dec_sec.iter() {
//             data.extend(chunk.iter());
//         }
//         Ok(data)
//     }
//     fn decrypt_bytes(&self, data: DataSeq) -> Result<DataSeq>;
// }
