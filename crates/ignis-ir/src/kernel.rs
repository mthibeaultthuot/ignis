use crate::Stmt;

#[derive(Clone, Debug)]
pub struct Kernel {
    pub block: KernelBlock,
}

impl Kernel {
    pub fn empty() -> Self {
        Self {
            block: KernelBlock { stmts: vec![] },
        }
    }
}

#[derive(Clone, Debug)]
pub struct KernelBlock {
    pub stmts: Vec<Stmt>,
}

impl KernelBlock {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }
}
