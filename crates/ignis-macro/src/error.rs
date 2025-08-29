use thiserror::Error;

#[derive(Error, Debug)]
pub enum MacroError {
    #[error("unknown data store error")]
    Unknown,
}
