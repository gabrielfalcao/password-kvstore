use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::iter::{Extend, IntoIterator, Iterator};

use serde::{Deserialize, Serialize};

use crate::traits::PlainBytes;
use crate::Result;

#[derive(Clone, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Data {
    pub bytes: Vec<u8>,
}
impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        if self.bytes.len() != other.bytes.len() {
            return false
        }
        let mut pos = 0;
        let len = self.bytes.len();
        while pos < len {
            if self.bytes[pos] != other.bytes[pos] {
                return false
            }
            pos += 1;
        }
        true
    }
}


impl Data {
    pub fn new(bytes: Vec<u8>) -> Data {
        Data { bytes }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn to_hex(&self, sep: &str, hint: bool) -> String {
        self.bytes
            .iter()
            .map(|o| format!("{}{:02x}", if hint { "0x" } else { "" }, o))
            .collect::<Vec<String>>()
            .join(sep)
    }

    pub fn from_hex(data: &str) -> Result<Data> {
        let data = hex::decode(data)?;
        Ok(Data::new(data))
    }

    pub fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        crate::to_flate_bytes(self)
    }

    pub fn from_deflate_bytes(bytes: &[u8]) -> Result<Data> {
        Ok(crate::from_deflate_bytes::<Data>(bytes)?)
    }

    pub fn iter(&self) -> DataIterator {
        DataIterator::new(self)
    }

    pub fn filter(self, predicate: impl FnMut(&u8) -> bool) -> Data {
        self.iter().filter(predicate).collect::<Data>()
    }

    pub fn map(self, predicate: impl FnMut(u8) -> u8) -> Data {
        self.iter().map(predicate).collect::<Data>()
    }

    pub fn difference(&self, other: &Data) -> Data {
        self.set().difference(&other.set()).cloned().collect()
    }

    pub fn intersection(&self, other: &Data) -> Data {
        self.set().intersection(&other.set()).cloned().collect()
    }

    pub fn contains(&mut self, byte: u8) -> bool {
        !self.bytes.iter().filter(|c| **c == byte).collect::<Vec<_>>().is_empty()
    }

    pub fn sort_by(&mut self, f: impl FnMut(&u8, &u8) -> Ordering) {
        self.bytes.sort_by(f)
    }

    pub fn get(&mut self, index: usize) -> Option<u8> {
        self.bytes.get(index).map(|byte| *byte)
    }

    pub fn push(&mut self, byte: u8) {
        self.bytes.push(byte)
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.bytes.pop()
    }

    pub fn set(&self) -> BTreeSet<u8> {
        let mut set = BTreeSet::new();
        for v in self.iter() {
            set.insert(v);
        }
        set
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn extend<T: Iterator<Item = u8>>(&mut self, iter: T) {
        self.bytes.extend(iter);
    }

    pub fn extended<T: Iterator<Item = u8>>(&self, iter: T) -> Data {
        let mut data = self.clone();
        data.extend(iter);
        data
    }

    pub fn then<T>(&self, mut no_more: impl FnMut(Self) -> T) -> Option<T> {
        if self.len() > 0 {
            Some(no_more(self.clone()))
        } else {
            None
        }
    }

    pub fn randomc<R: rand::CryptoRng + rand::RngCore>(mut random: R, length: usize) -> Data {
        let mut bytes: Vec<u8> = Vec::with_capacity(length);
        bytes.resize(length, 0);
        random.fill_bytes(&mut bytes);
        Data::new(bytes)
    }
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "data![{}]", self.to_hex(", ", true))
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_hex("", false))
    }
}

impl PlainBytes for Data {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Data {
        Data::new(bytes.into())
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DataIterator {
    data: Data,
    pos: usize,
}

impl DataIterator {
    pub fn new(data: &Data) -> DataIterator {
        DataIterator {
            data: data.clone(),
            pos: 0,
        }
    }
}

impl Iterator for DataIterator {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let item = self.data.get(self.pos);
        self.pos += 1;
        item
    }
}

impl IntoIterator for Data {
    type IntoIter = DataIterator;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
