use thiserror::Error;

#[derive(Error, Debug)]
pub enum MacroError {
    #[error("Unsupported expression type")]
    UnsupportedExprType,
}
