use std::mem;

use crate::{Expr, ExprStmt, Kernel, KernelBlock, Stmt};
use syn::ItemFn;

use crate::{LowerCtx, error::MacroError};

/// Parser for Ignis kernels.
/// Parses syn token into Ignis AST.
#[allow(dead_code)]
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
        // Syn ItemFn Block Stmt is actaly box so mem::take(&mut T)
        let stmts = mem::take(&mut self.item_fn.block.stmts)
            .into_iter()
            .map(|item| match item {
                syn::Stmt::Expr(expr, _) => {
                    let new_expr = Expr::parse_from_syn(&expr, &mut self.ctx)?;
                    Ok(Stmt::Expr(ExprStmt {
                        dst: Some(self.ctx.fresh()),
                        expr: new_expr,
                    }))
                }
                _ => panic!("Unsupported statement in ignis"),
            })
            .collect::<Result<Vec<_>, MacroError>>()?;
        Ok(Kernel {
            block: KernelBlock::new(stmts),
        })
    }
}
