use ignis_ir::{Kernel, Type};
use syn::{ExprBinary, ItemFn};

use crate::{error::MacroError, lowering::LowerCtx};

pub struct Parser {
    item_fn: ItemFn,
    ctx: LowerCtx,
}

impl Parser {
    pub fn new(item_fn: ItemFn) -> Self {
        Self {
            item_fn,
            ctx: LowerCtx::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Kernel, MacroError> {
        let mut kernel = Kernel::empty();

        let stmts = self.item_fn.block.stmts.clone();
        for stmt in stmts {
            let new_expr = match stmt {
                syn::Stmt::Expr(expr, _) => self.parse_expr(&expr),
                _ => {
                    return Err(MacroError::Parse(format!(
                        "unsupported statement in ignis : {:?}",
                        stmt,
                    )));
                }
            };
            kernel
                .block
                .stmts
                .push(ignis_ir::Stmt::Expr(ignis_ir::ExprStmt {
                    dst: Some(self.ctx.fresh()),
                    expr: new_expr,
                }));
        }
        Ok(kernel)
    }

    pub fn parse_expr(&mut self, expr: &syn::Expr) -> ignis_ir::Expr {
        match expr {
            syn::Expr::Binary(ExprBinary {
                left, op, right, ..
            }) => {
                let lhs_name = match &**left {
                    syn::Expr::Path(p) => self.lower_path(p),
                    other => panic!("lhs not a path: {:?}", other),
                };
                let rhs_name = match &**right {
                    syn::Expr::Path(p) => self.lower_path(p),
                    other => panic!("rhs not a path: {:?}", other),
                };
                let lhs = self.ctx.get_or_create(&lhs_name);
                let rhs = self.ctx.get_or_create(&rhs_name);

                let op = self.parse_binop(op);

                ignis_ir::Expr::Binary {
                    op: op,
                    lhs: lhs,
                    rhs: rhs,
                    // skip for now
                    ty: Type::F32,
                }
            }
            _ => panic!("can't parse the Expr from syn"),
        }
    }

    pub fn parse_binop(&self, binop: &syn::BinOp) -> ignis_ir::BinOp {
        match binop {
            syn::BinOp::Add(_) => ignis_ir::BinOp::Add,
            syn::BinOp::Mul(_) => ignis_ir::BinOp::Mul,
            _ => panic!("BinOp not found"),
        }
    }

    fn lower_path(&self, expr: &syn::ExprPath) -> String {
        expr.path
            .get_ident()
            .expect("expected simple ident")
            .to_string()
    }
}
