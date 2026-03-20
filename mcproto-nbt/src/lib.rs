use thiserror::Error;

pub mod nbt;

#[derive(Debug, Error)]
pub enum NbtError {
    #[error("Unknown tag id {0}")]
    UnknownTag(u8),

    #[error("Root tag is not a compound")]
    InvalidRoot,

    #[error("IO error: {0}")]
    Io(String),

    #[error("Invalid UTF-8 string: {0}")]
    Utf8(String),
}
