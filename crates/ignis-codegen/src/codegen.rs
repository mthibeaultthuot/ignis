use ignis_ir::{DType, Expr, ExprStmt, Kernel, SSAVar, Stmt};

pub trait Codegen {
    fn lower_kernel(kernel: &Kernel) -> String;

    fn lower_stmt(stmt: &Stmt) -> String;

    fn lower_expr(dst: &Option<SSAVar>, expr: &Expr) -> String;

    fn lower_type(ty: &ignis_ir::Type) -> String;
}

fn lower_for_backend<B: Codegen>(kernel: &Kernel) -> String {
    B::lower_kernel(kernel)
}
