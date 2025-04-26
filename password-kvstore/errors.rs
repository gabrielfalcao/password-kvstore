use std::fmt::Display;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Error {
    AlreadyExists(String),
    NotFound(String),
    DecodingError(String),
    DecryptionError(String),
    DeserializationError(String),
    EncodingError(String),
    EncryptionError(String),
    HexDecodeError(String),
    IOError(String),
    InvalidKeyError(String),
    InvalidUtf8(String),
    PKCS1Error(String),
    PKCS8Error(String),
    ParseIntError(String),
    RSAError(String),
    StorageError(String),
    PasswordHashingError(String),
}

impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 2)?;
        s.serialize_field("variant", &self.variant())?;
        s.serialize_field("message", &format!("{}", self))?;
        s.end()
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Self::NotFound(e) => e.to_string(),
                Self::DecodingError(e) => e.to_string(),
                Self::DecryptionError(e) => e.to_string(),
                Self::DeserializationError(e) => e.to_string(),
                Self::EncodingError(e) => e.to_string(),
                Self::EncryptionError(e) => e.to_string(),
                Self::AlreadyExists(e) => e.to_string(),
                Self::HexDecodeError(e) => e.to_string(),
                Self::IOError(e) => e.to_string(),
                Self::InvalidKeyError(e) => e.to_string(),
                Self::InvalidUtf8(e) => e.to_string(),
                Self::PKCS1Error(e) => e.to_string(),
                Self::PKCS8Error(e) => e.to_string(),
                Self::ParseIntError(e) => e.to_string(),
                Self::RSAError(e) => e.to_string(),
                Self::StorageError(e) => e.to_string(),
                Self::PasswordHashingError(e) => e.to_string(),
            }
        )
    }
}
impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::AlreadyExists(_) => "AlreadyExists",
            Error::NotFound(_) => "NotFound",
            Error::DecodingError(_) => "DecodingError",
            Error::DecryptionError(_) => "DecryptionError",
            Error::DeserializationError(_) => "DeserializationError",
            Error::EncodingError(_) => "EncodingError",
            Error::EncryptionError(_) => "EncryptionError",
            Error::HexDecodeError(_) => "HexDecodeError",
            Error::IOError(_) => "IOError",
            Error::InvalidKeyError(_) => "InvalidKeyError",
            Error::InvalidUtf8(_) => "InvalidUtf8",
            Error::PKCS1Error(_) => "PKCS1Error",
            Error::PKCS8Error(_) => "PKCS8Error",
            Error::ParseIntError(_) => "ParseIntError",
            Error::RSAError(_) => "RSAError",
            Error::StorageError(_) => "StorageError",
            Error::PasswordHashingError(_) => "PasswordHashingError",
        }
        .to_string()
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<iocore::Error> for Error {
    fn from(e: iocore::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseIntError(format!("{}", e))
    }
}
impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::InvalidUtf8(format!("{}", e))
    }
}
impl From<rsa::Error> for Error {
    fn from(e: rsa::Error) -> Self {
        Error::RSAError(format!("{}", e))
    }
}
impl From<rsa::pkcs8::Error> for Error {
    fn from(e: rsa::pkcs8::Error) -> Self {
        Error::PKCS8Error(format!("{}", e))
    }
}
impl From<rsa::pkcs1::Error> for Error {
    fn from(e: rsa::pkcs1::Error) -> Self {
        Error::PKCS1Error(format!("{}", e))
    }
}
impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::HexDecodeError(format!("{}", e))
    }
}
impl From<Box<bincode::ErrorKind>> for Error {
    fn from(e: Box<bincode::ErrorKind>) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<argon2_kdf::Argon2Error> for Error {
    fn from(e: argon2_kdf::Argon2Error) -> Self {
        Error::PasswordHashingError(format!("{}", e))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
