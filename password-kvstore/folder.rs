use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Entry, Error, PlainBytes, Result};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Folder {
    pub name: String,
    pub entries: BTreeMap<String, Entry>,
}
impl PlainBytes for Folder {}
impl Folder {
    pub fn add_entry(&mut self, entry: Entry) -> Result<Entry> {
        let entry = Into::<Entry>::into(entry);
        let name = entry.name.to_string();
        if let Some(_) = self.entries.get(&name) {
            return Err(Error::AlreadyExists(format!("entry {:#?} already exists", name)));
        } else {
            self.entries.insert(name.to_string(), entry.clone());
        }
        Ok(entry)
    }

    pub fn update_entry(&mut self, entry: &Entry) -> Result<()> {
        let name = entry.name.to_string();
        if self.entries.contains_key(&name) {
            self.entries.insert(name, entry.clone());
            Ok(())
        } else {
            Err(Error::NotFound(format!("no entry found with name {:#?} ", name)))
        }
    }

    pub fn get(&self, name: &str) -> Result<Entry> {
        let name = name.to_string();
        match self.entries.get(&name) {
            Some(entry) => Ok(entry.clone()),
            None => Err(Error::NotFound(format!("no entry found with name {:#?} ", name))),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{Entry, Folder, Result, Secret};
    #[test]
    fn test_folder() -> Result<()> {
        let mut folder = Folder::default();
        let mut entry = folder.add_entry(Entry::from("entry"))?;
        entry.password = Secret::from("entry");
        folder.update_entry(&entry)?;
        assert_eq!(folder.get("entry")?, entry);
        Ok(())
    }
}
