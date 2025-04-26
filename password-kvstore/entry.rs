use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{PlainBytes, Secret};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub username: String,
    pub password: Secret,
    pub description: String,
    pub email: String,
    pub urls: Vec<String>,
    pub attributes: BTreeMap<String, Secret>,
}
impl From<&str> for Entry {
    fn from(name: &str) -> Entry {
        Entry::new(name)
    }
}
impl From<String> for Entry {
    fn from(name: String) -> Entry {
        Entry::new(name.as_str())
    }
}
impl PlainBytes for Entry {}
impl Entry {
    pub fn new(name: &str) -> Entry {
        let mut entry = Entry::default();
        entry.name = name.to_string();
        entry
    }
}

#[cfg(test)]
mod tests {
    use crate::{Entry, PlainBytes, Result, Secret};
    #[test]
    fn test_entry_to_plain_bytes() -> Result<()> {
        let mut entry = Entry::new("entry");
        entry.password = Secret::from("entry");

        assert_eq!(
            entry.to_plain_bytes(),
            vec![
                5, 0, 0, 0, 0, 0, 0, 0, 101, 110, 116, 114, 121, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0,
                0, 0, 0, 0, 0, 101, 110, 116, 114, 121, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        Ok(())
    }
    #[test]
    fn test_entry_to_flate_bytes() -> Result<()> {
        let mut entry = Entry::new("entry");
        entry.password = Secret::from("entry");

        assert_eq!(
            entry.to_flate_bytes()?,
            vec![
                133, 136, 161, 13, 0, 0, 8, 195, 102, 118, 42, 22, 65, 48, 124, 143, 0, 18, 220,
                170, 218, 18, 131, 121, 70, 173, 131, 127, 94, 40, 26
            ]
        );
        Ok(())
    }
    #[test]
    fn test_entry_from_plain_bytes() -> Result<()> {
        let entry_from_bytes = Entry::from_plain_bytes(&[
            5, 0, 0, 0, 0, 0, 0, 0, 101, 110, 116, 114, 121, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0,
            0, 0, 0, 101, 110, 116, 114, 121, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])?;
        let mut entry = Entry::new("entry");
        entry.password = Secret::from("entry");

        assert_eq!(entry_from_bytes, entry);
        Ok(())
    }
    #[test]
    fn test_entry_from_flate_bytes() -> Result<()> {
        let entry_from_bytes = Entry::from_deflate_bytes(&[
            133, 136, 161, 13, 0, 0, 8, 195, 102, 118, 42, 22, 65, 48, 124, 143, 0, 18, 220, 170,
            218, 18, 131, 121, 70, 173, 131, 127, 94, 40, 26,
        ])?;
        let mut entry = Entry::new("entry");
        entry.password = Secret::from("entry");

        assert_eq!(entry_from_bytes, entry);
        Ok(())
    }
}
