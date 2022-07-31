use std::env;
use std::io;

#[derive(Debug, thiserror::Error)]
#[error("Invalid RUSTEE_MODE `{0}`")]
pub struct InvalidRusteeMode(pub String);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    InvalidRusteeMode(#[from] InvalidRusteeMode),

    #[error("{0}")]
    IoError(#[from] io::Error),

    #[error("{0}")]
    VarError(#[from] env::VarError),

    #[error("output file path is not provided")]
    FileNotProvided,
}