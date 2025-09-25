use thiserror::Error;

#[derive(Error, Debug)]
pub enum MacroError {
    #[error("data store disconnected")]
    Parse(String),
    #[error("Unsupported Stmt type")]
    UnsupportedStmtType(String),
    #[error("Unsupported expression type")]
    UnsupportedExprType,
}
