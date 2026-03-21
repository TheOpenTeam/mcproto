use thiserror::Error;

pub mod nbt;
pub mod templates;
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

    #[error("Unknown compound {0}")]
    UnknownCompound(String)
}
