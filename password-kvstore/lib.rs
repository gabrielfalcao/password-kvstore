pub(crate) mod data;
pub(crate) mod entry;
pub(crate) mod errors;
pub(crate) mod folder;
pub(crate) mod secret;
pub(crate) mod secret_box;
pub(crate) mod traits;
pub(crate) mod utils;
pub(crate) mod tool;

pub use data::{Data, DataSeq, DataSeqIterator};
pub use entry::Entry;
pub use errors::{Error, Result};
pub use folder::Folder;
// pub use password::{password_decrypt_bytes, password_encrypt_bytes};
pub use secret::Secret;
pub use secret_box::SecretBox;
pub use traits::PlainBytes;
pub use utils::{
    chunk_padded, discharge, drop, from_deflate_bytes, rev, scrub_with_byte, to_flate_bytes, xor,
    xor_ip, zerofill,
};
pub use tool::{
    Chacha20Tool, CipherText
};
