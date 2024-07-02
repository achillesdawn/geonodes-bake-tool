use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetaReadError {
    #[error("File read error")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("Deserializing error")]
    Deserialize {
        #[from]
        source: serde_json::Error,
    },
    #[error("Item not found")]
    ItemNotFound,
    #[error("Attribute not found")]
    AttributeNotFound,
    #[error("Parsing Blob name error")]
    ParseIntError,
}
