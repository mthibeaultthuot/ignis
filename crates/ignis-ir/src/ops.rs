#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Not,
    Abs,
    Sqrt,
    Rsqrt,
    Exp,
    Log,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Fma,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ReduceOp {
    Add,
    Mul,
    Min,
    Max,
    And,
    Or,
    Xor,
}
