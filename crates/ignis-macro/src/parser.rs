use crate::{Expr, ExprStmt, Kernel, Stmt, Type};
use syn::{ExprBinary, ItemFn};

use crate::{LowerCtx, error::MacroError};

/// Parser for Ignis kernels.
/// Parses syn token into Ignis AST.
#[derive(Debug, Clone)]
pub struct Parser {
    // the parsed Rust function AST
    item_fn: ItemFn,
    // Context for naming SSA
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
                syn::Stmt::Expr(expr, _) => Expr::parse_from_syn(&expr, &mut self.ctx)?,
                _ => {
                    return Err(MacroError::UnsupportedStmtType(format!(
                        "Unsupported statement in ignis : {:?}",
                        stmt,
                    )));
                }
            };
            kernel.block.stmts.push(Stmt::Expr(ExprStmt {
                dst: Some(self.ctx.fresh()),
                expr: new_expr,
            }));
        }
        Ok(kernel)
    }
}
