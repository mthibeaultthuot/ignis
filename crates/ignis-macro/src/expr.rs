use crate::{BinOp, BufferId, Builtin, LowerCtx, ReduceOp, SSAVar, Type, UnOp, error::MacroError};

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

impl Expr {
    pub fn parse_from_syn(expr: &syn::Expr, ctx: &mut LowerCtx) -> Result<Expr, MacroError> {
        match expr {
            syn::Expr::Binary(syn::ExprBinary {
                left, op, right, ..
            }) => {
                let lhs_name = match &**left {
                    syn::Expr::Path(p) => Expr::lower_path(p),
                    other => panic!("lhs not a path: {:?}", other),
                };
                let rhs_name = match &**right {
                    syn::Expr::Path(p) => Expr::lower_path(p),
                    other => panic!("rhs not a path: {:?}", other),
                };
                let lhs = ctx.get_or_create(&lhs_name);
                let rhs = ctx.get_or_create(&rhs_name);

                let op = Expr::parse_binop(op);

                Ok(Expr::Binary {
                    op,
                    lhs,
                    rhs,
                    // Skip for now we
                    // TODO : handling type inference
                    ty: Type::F32,
                })
            }
            _ => Err(MacroError::UnsupportedExprType),
        }
    }

    pub fn parse_binop(binop: &syn::BinOp) -> crate::BinOp {
        match binop {
            syn::BinOp::Add(_) => BinOp::Add,
            syn::BinOp::Mul(_) => BinOp::Mul,
            _ => panic!("BinOp not found"),
        }
    }

    fn lower_path(expr: &syn::ExprPath) -> String {
        expr.path
            .get_ident()
            .expect("expected simple ident")
            .to_string()
    }
}
