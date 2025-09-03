use crate::{BinOp, BufferId, Builtin, ReduceOp, SSAVar, Type, UnOp};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Expr {
    ConstI(i64, Type),
    ConstF(f64, Type),
    ConstBool(bool),

    Var(SSAVar, Type),

    Builtin(Builtin, Type),

    Unary {
        op: UnOp,
        x: SSAVar,
        ty: Type,
    },
    Binary {
        op: BinOp,
        lhs: SSAVar,
        rhs: SSAVar,
        ty: Type,
    },

    Load {
        buffer: BufferId,
        index: SSAVar,
        ty: Type,
    },

    Reduce {
        op: ReduceOp,
        lhs: SSAVar,
        rhs: SSAVar,
        ty: Type,
    },

    Select {
        cond: SSAVar,
        t: SSAVar,
        f: SSAVar,
        ty: Type,
    },
}
