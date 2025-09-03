use std::fmt::Binary;

use ignis_ir::{DType, Expr, ExprStmt, Kernel, SSAVar, Stmt};

use crate::codegen::Codegen;

struct MetalCodegen;

/// for reference of metal language
/// https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf
impl Codegen for MetalCodegen {
    fn lower_kernel(kernel: &Kernel) -> String {
        let mut body = String::new();

        for stmt in &kernel.block.stmts {
            let line = MetalCodegen::lower_stmt(stmt);
            body.push_str(&line);
            body.push('\n');
        }
        // Hardcoded for now. Not clean yet, but at least lower_stmt for binary ops works.
        format!(
            r#"#include <metal_stdlib>
        using namespace metal;

        kernel void main0(device const float* lhs [[ buffer(0) ]],
                          device const float* rhs [[ buffer(1) ]],
                          device float* out [[ buffer(2) ]],
                          uint id [[ thread_position_in_grid ]]) {{
            float v0 = lhs[id];
            float v1 = rhs[id];
            {body}
            out[id] = v2;
        }}"#
        )
    }

    fn lower_stmt(stmt: &Stmt) -> String {
        match stmt {
            Stmt::Expr(ExprStmt { dst, expr }) => Self::lower_expr(dst, expr),
            _ => panic!("stmt no implemented"),
        }
    }

    fn lower_expr(dst: &Option<SSAVar>, expr: &Expr) -> String {
        match expr {
            Expr::Binary { op, lhs, rhs, ty } => {
                if let Some(dst) = dst {
                    format!("  {} {}={}{}{}", Self::lower_type(ty), dst, lhs, op, rhs)
                } else {
                    format!("{}{}{}", lhs, op, rhs)
                }
            }

            _ => panic!("expr not implemented yet"),
        }
    }

    fn lower_type(ty: &ignis_ir::Type) -> String {
        let base = match ty.scalar {
            DType::I32 => "int",
            DType::I64 => "long",
            DType::U32 => "uint",
            DType::F16 => "half",
            DType::F32 => "float",
            DType::F64 => "double",
            DType::Bool => "bool",
        };
        if ty.lanes == 1 {
            base.to_string()
        } else {
            format!("{}{}", base, ty.lanes)
        }
    }
}

#[cfg(test)]
mod test {
    use ignis_ir::{BinOp, DType, ExprStmt, KernelBlock, SSAVar, Type};

    use super::*;

    #[test]
    fn test_add() {
        let kernel = Kernel {
            block: KernelBlock {
                stmts: vec![Stmt::Expr(ExprStmt {
                    dst: Some(SSAVar(2)),
                    expr: Expr::Binary {
                        op: BinOp::Add,
                        lhs: SSAVar(0),
                        rhs: SSAVar(1),
                        ty: Type {
                            scalar: DType::F32,
                            lanes: 1,
                        },
                    },
                })],
            },
        };
        let output = MetalCodegen::lower_kernel(&kernel);
        println!("{}", output);
    }
}
