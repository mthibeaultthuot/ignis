use crate::Stmt;

#[derive(Clone, Debug)]
pub struct Kernel {
    pub block: KernelBlock,
}
#[derive(Clone, Debug)]
pub struct KernelBlock {
    pub stmts: Vec<Stmt>,
}
