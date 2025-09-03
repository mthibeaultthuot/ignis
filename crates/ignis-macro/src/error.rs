use thiserror::Error;

#[derive(Error, Debug)]
pub enum MacroError {
    #[error("data store disconnected")]
    Parse(String),
}
