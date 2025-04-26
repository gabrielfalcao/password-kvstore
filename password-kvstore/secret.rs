use std::fmt::{Debug, Display};

use sanitation::SString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Secret {
    data: Vec<u8>,
    len: usize,
}
impl Secret {
    pub fn from_plaintext(plaintext: impl Display) -> Secret {
        let data = plaintext.to_string().as_bytes().to_vec();
        let len = data.len();
        Secret { data, len }
    }

    pub fn plaintext(&self) -> String {
        SString::new(&self.data).unchecked_safe()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
impl From<&str> for Secret {
    fn from(t: &str) -> Secret {
        Secret::from_plaintext(t)
    }
}
impl From<String> for Secret {
    fn from(t: String) -> Secret {
        Secret::from_plaintext(t)
    }
}
impl Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (0..self.len).map(|_| '*').collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use crate::Secret;

    #[test]
    fn test_secret() {
        let secret = Secret::from("secret");
        assert_eq!(secret.to_string(), "******");
    }
}
