use crate::{
    AddressSpace, AtomicOp, BarrierScope, BufferId, Expr, KernelBlock, MemFenceScope, SSAVar, Type,
};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Stmt {
    Local(LocalStmt),
    Item(ItemStmt),
    Expr(ExprStmt),
    Macro(MacroStmt),
}

#[derive(Clone, Debug)]
pub struct LocalStmt {
    pub dst: SSAVar,
    pub init: Expr,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ItemStmt {
    BufferDecl {
        id: BufferId,
        name: String,
        addr_space: AddressSpace,
        elem_ty: Type,
        shape: Vec<u32>,
        strides: Option<Vec<u32>>,
        is_param: bool,
    },
}

#[derive(Clone, Debug)]
pub struct ExprStmt {
    pub dst: Option<SSAVar>,
    pub expr: Expr,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum MacroStmt {
    Barrier {
        scope: BarrierScope,
    },
    MemFence {
        scope: MemFenceScope,
    },
    Atomic {
        op: AtomicOp,
        buffer: BufferId,
        index: SSAVar,
        value: SSAVar,
    },
    Store {
        buffer: BufferId,
        index: SSAVar,
        value: SSAVar,
    },
    If {
        cond: SSAVar,
        then_blk: KernelBlock,
        else_blk: Option<KernelBlock>,
    },
    For {
        var: SSAVar,
        start: SSAVar,
        end: SSAVar,
        step: SSAVar,
        body: KernelBlock,
    },
}
