use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Chacha20Tool, CipherText, Entry, Error, PlainBytes, Result, SecretBox};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Folder {
    pub name: String,
    pub entries: BTreeMap<String, SecretBox>,
    pub nonces: BTreeMap<String, [u8; 12]>,
}
impl PlainBytes for Folder {}
impl Folder {
    fn encrypt_and_insert_entry(&mut self, entry: &Entry, tool: &Chacha20Tool) -> Result<()> {
        let entry = entry.clone();
        let entry_ciphertext = tool.encrypt(&entry.to_flate_bytes()?)?;
        self.entries.insert(
            entry.name.to_string(),
            SecretBox::close(&entry_ciphertext.ciphertext().bytes)?,
        );
        self.nonces.insert(entry.name.to_string(), entry_ciphertext.nonce());
        Ok(())
    }

    pub fn add_entry(&mut self, entry: Entry, tool: &Chacha20Tool) -> Result<Entry> {
        let entry = Into::<Entry>::into(entry);
        let name = entry.name.to_string();
        if let Some(_) = self.entries.get(&name) {
            return Err(Error::AlreadyExists(format!("entry {:#?} already exists", name)));
        } else {
            self.encrypt_and_insert_entry(&entry, tool)?;
        }
        Ok(entry)
    }

    pub fn update_entry(&mut self, entry: &Entry, tool: &Chacha20Tool) -> Result<()> {
        let name = entry.name.to_string();
        if self.entries.contains_key(&name) {
            self.encrypt_and_insert_entry(entry, tool)?;
            Ok(())
        } else {
            Err(Error::NotFound(format!("no entry found with name {:#?}", name)))
        }
    }

    pub fn get_nonce(&self, name: &str) -> Result<[u8; 12]> {
        let name = name.to_string();
        match self.nonces.get(&name) {
            Some(nonce) => Ok(nonce.clone()),
            None => Err(Error::NotFound(format!("no entry (nonce) found with name {:#?}", name))),
        }
    }

    pub fn get(&self, name: &str, tool: &Chacha20Tool) -> Result<Entry> {
        let name = name.to_string();
        let secret_box = match self.entries.get(&name) {
            Some(ciphertext) => ciphertext,
            None => {
                return Err(Error::NotFound(format!("no entry found with name {:#?}", name)));
            },
        };
        let nonce = self.get_nonce(&name)?;

        let entry_ciphertext = secret_box.open()?;
        let ciphertext = CipherText::new(entry_ciphertext.clone(), nonce);
        let bytes = tool.decrypt(&ciphertext)?;
        let entry = Entry::from_deflate_bytes(&bytes)?;
        Ok(entry)
    }

    pub fn delete(&mut self, name: &str) -> Result<bool> {
        let name = name.to_string();
        let entry_deleted = self.entries.remove(&name).map(|_| true).unwrap_or_else(|| false);
        if entry_deleted {
            self.nonces.remove(&name);
            Ok(true)
        } else {
            Err(Error::NotFound(format!("no entry found with name {:#?}", name)))
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{Chacha20Tool, Entry, Error, Folder, Result, Secret};
    #[test]
    fn test_folder() -> Result<()> {
        let tool = Chacha20Tool::new("password", 600)?;
        let mut folder = Folder::default();
        let mut entry = folder.add_entry(Entry::from("entry"), &tool)?;
        entry.password = Secret::from("entry");
        folder.update_entry(&entry, &tool)?;
        assert_eq!(folder.get("entry", &tool)?, entry);
        folder.delete(entry.name.as_str())?;
        assert_eq!(
            folder.get("entry", &tool),
            Err(Error::NotFound(format!("no entry found with name {:#?}", "entry")))
        );
        Ok(())
    }
}
