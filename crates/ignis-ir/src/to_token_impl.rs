use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{
    AddressSpace, AtomicOp, BarrierScope, BinOp, BufferId, Builtin, DType, Expr, ExprStmt,
    ItemStmt, Kernel, KernelBlock, LocalStmt, MacroStmt, MemFenceScope, ReduceOp, SSAVar, Stmt,
    Type, UnOp,
};

impl ToTokens for Builtin {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            Builtin::ThreadIdxX => quote!(ignis_ir::Builtin::ThreadIdxX),
            Builtin::ThreadIdxY => quote!(ignis_ir::Builtin::ThreadIdxY),
            Builtin::ThreadIdxZ => quote!(ignis_ir::Builtin::ThreadIdxZ),
            Builtin::BlockIdxX => quote!(ignis_ir::Builtin::BlockIdxX),
            Builtin::BlockIdxY => quote!(ignis_ir::Builtin::BlockIdxY),
            Builtin::BlockIdxZ => quote!(ignis_ir::Builtin::BlockIdxZ),
            Builtin::BlockDimX => quote!(ignis_ir::Builtin::BlockDimX),
            Builtin::BlockDimY => quote!(ignis_ir::Builtin::BlockDimY),
            Builtin::BlockDimZ => quote!(ignis_ir::Builtin::BlockDimZ),
            Builtin::GridDimX => quote!(ignis_ir::Builtin::GridDimX),
            Builtin::GridDimY => quote!(ignis_ir::Builtin::GridDimY),
            Builtin::GridDimZ => quote!(ignis_ir::Builtin::GridDimZ),
            Builtin::LaneId => quote!(ignis_ir::Builtin::LaneId),
            Builtin::WarpSize => quote!(ignis_ir::Builtin::WarpSize),
        };
        tokens.extend(q);
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            Expr::ConstI(v, ty) => {
                quote!(ignis_ir::Expr::ConstI(#v, #ty))
            }
            Expr::ConstF(v, ty) => {
                quote!(ignis_ir::Expr::ConstF(#v, #ty))
            }
            Expr::ConstBool(v) => {
                quote!(ignis_ir::Expr::ConstBool(#v))
            }
            Expr::Var(var, ty) => {
                quote!(ignis_ir::Expr::Var(#var, #ty))
            }
            Expr::Builtin(b, ty) => {
                quote!(ignis_ir::Expr::Builtin(#b, #ty))
            }
            Expr::Unary { op, x, ty } => {
                quote!(ignis_ir::Expr::Unary {
                    op: #op,
                    x: #x,
                    ty: #ty,
                })
            }
            Expr::Binary { op, lhs, rhs, ty } => {
                quote!(ignis_ir::Expr::Binary {
                    op: #op,
                    lhs: #lhs,
                    rhs: #rhs,
                    ty: #ty,
                })
            }
            Expr::Load { buffer, index, ty } => {
                quote!(ignis_ir::Expr::Load {
                    buffer: #buffer,
                    index: #index,
                    ty: #ty,
                })
            }
            Expr::Reduce { op, lhs, rhs, ty } => {
                quote!(ignis_ir::Expr::Reduce {
                    op: #op,
                    lhs: #lhs,
                    rhs: #rhs,
                    ty: #ty,
                })
            }
            Expr::Select { cond, t, f, ty } => {
                quote!(ignis_ir::Expr::Select {
                    cond: #cond,
                    t: #t,
                    f: #f,
                    ty: #ty,
                })
            }
        };
        tokens.extend(q);
    }
}

impl ToTokens for Kernel {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let block = &self.block;
        tokens.extend(quote! {
            ignis_ir::Kernel {
                block: #block,
            }
        });
    }
}

impl ToTokens for KernelBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let stmts = &self.stmts;
        tokens.extend(quote! {
            ignis_ir::KernelBlock {
                stmts: vec![#(#stmts),*],
            }
        });
    }
}

impl ToTokens for AtomicOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            AtomicOp::Add => quote!(ignis_ir::AtomicOp::Add),
            AtomicOp::Sub => quote!(ignis_ir::AtomicOp::Sub),
            AtomicOp::Min => quote!(ignis_ir::AtomicOp::Min),
            AtomicOp::Max => quote!(ignis_ir::AtomicOp::Max),
            AtomicOp::And => quote!(ignis_ir::AtomicOp::And),
            AtomicOp::Or => quote!(ignis_ir::AtomicOp::Or),
            AtomicOp::Xor => quote!(ignis_ir::AtomicOp::Xor),
            AtomicOp::Exchange => quote!(ignis_ir::AtomicOp::Exchange),
            AtomicOp::CompareExchange => quote!(ignis_ir::AtomicOp::CompareExchange),
        };
        tokens.extend(q);
    }
}

impl ToTokens for AddressSpace {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            AddressSpace::Global => quote!(ignis_ir::AddressSpace::Global),
            AddressSpace::Shared => quote!(ignis_ir::AddressSpace::Shared),
            AddressSpace::Local => quote!(ignis_ir::AddressSpace::Local),
            AddressSpace::Const => quote!(ignis_ir::AddressSpace::Const),
        };
        tokens.extend(q);
    }
}

impl ToTokens for BarrierScope {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            BarrierScope::ThreadGroup => quote!(ignis_ir::BarrierScope::ThreadGroup),
            BarrierScope::Device => quote!(ignis_ir::BarrierScope::Device),
        };
        tokens.extend(q);
    }
}

impl ToTokens for MemFenceScope {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            MemFenceScope::ThreadGroup => quote!(ignis_ir::MemFenceScope::ThreadGroup),
            MemFenceScope::Device => quote!(ignis_ir::MemFenceScope::Device),
            MemFenceScope::System => quote!(ignis_ir::MemFenceScope::System),
        };
        tokens.extend(q);
    }
}

impl ToTokens for UnOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            UnOp::Neg => quote!(ignis_ir::UnOp::Neg),
            UnOp::Not => quote!(ignis_ir::UnOp::Not),
            UnOp::Abs => quote!(ignis_ir::UnOp::Abs),
            UnOp::Sqrt => quote!(ignis_ir::UnOp::Sqrt),
            UnOp::Rsqrt => quote!(ignis_ir::UnOp::Rsqrt),
            UnOp::Exp => quote!(ignis_ir::UnOp::Exp),
            UnOp::Log => quote!(ignis_ir::UnOp::Log),
        };
        tokens.extend(q);
    }
}

impl ToTokens for BinOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            BinOp::Add => quote!(ignis_ir::BinOp::Add),
            BinOp::Sub => quote!(ignis_ir::BinOp::Sub),
            BinOp::Mul => quote!(ignis_ir::BinOp::Mul),
            BinOp::Div => quote!(ignis_ir::BinOp::Div),
            BinOp::Rem => quote!(ignis_ir::BinOp::Rem),
            BinOp::And => quote!(ignis_ir::BinOp::And),
            BinOp::Or => quote!(ignis_ir::BinOp::Or),
            BinOp::Xor => quote!(ignis_ir::BinOp::Xor),
            BinOp::Shl => quote!(ignis_ir::BinOp::Shl),
            BinOp::Shr => quote!(ignis_ir::BinOp::Shr),
            BinOp::Eq => quote!(ignis_ir::BinOp::Eq),
            BinOp::Ne => quote!(ignis_ir::BinOp::Ne),
            BinOp::Lt => quote!(ignis_ir::BinOp::Lt),
            BinOp::Le => quote!(ignis_ir::BinOp::Le),
            BinOp::Gt => quote!(ignis_ir::BinOp::Gt),
            BinOp::Ge => quote!(ignis_ir::BinOp::Ge),
            BinOp::Fma => quote!(ignis_ir::BinOp::Fma),
        };
        tokens.extend(q);
    }
}

impl ToTokens for ReduceOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            ReduceOp::Add => quote!(ignis_ir::ReduceOp::Add),
            ReduceOp::Mul => quote!(ignis_ir::ReduceOp::Mul),
            ReduceOp::Min => quote!(ignis_ir::ReduceOp::Min),
            ReduceOp::Max => quote!(ignis_ir::ReduceOp::Max),
            ReduceOp::And => quote!(ignis_ir::ReduceOp::And),
            ReduceOp::Or => quote!(ignis_ir::ReduceOp::Or),
            ReduceOp::Xor => quote!(ignis_ir::ReduceOp::Xor),
        };
        tokens.extend(q);
    }
}

impl ToTokens for Stmt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Stmt::Local(s) => s.to_tokens(tokens),
            Stmt::Item(s) => s.to_tokens(tokens),
            Stmt::Expr(s) => s.to_tokens(tokens),
            Stmt::Macro(s) => s.to_tokens(tokens),
        }
    }
}

impl ToTokens for LocalStmt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dst = &self.dst;
        let init = &self.init;
        tokens.extend(quote! {
            ignis_ir::Stmt::Local(ignis_ir::LocalStmt {
                dst: #dst,
                init: #init,
            })
        });
    }
}

impl ToTokens for ItemStmt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ItemStmt::BufferDecl {
                id,
                name,
                addr_space,
                elem_ty,
                shape,
                strides,
                is_param,
            } => {
                let shape_tokens = quote! { vec![#(#shape),*] };

                let strides_tokens = if let Some(s) = strides {
                    let elems = quote! { vec![#(#s),*] };
                    quote! { Some(#elems) }
                } else {
                    quote! { None }
                };

                tokens.extend(quote! {
                    ignis_ir::Stmt::Item(
                        ignis_ir::ItemStmt::BufferDecl {
                            id: #id,
                            name: #name.to_string(),
                            addr_space: #addr_space,
                            elem_ty: #elem_ty,
                            shape: #shape_tokens,
                            strides: #strides_tokens,
                            is_param: #is_param,
                        }
                    )
                });
            }
        }
    }
}

impl ToTokens for ExprStmt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dst_tokens = if let Some(d) = &self.dst {
            quote! { Some(#d) }
        } else {
            quote! { None }
        };
        let expr = &self.expr;
        tokens.extend(quote! {
            ignis_ir::Stmt::Expr(ignis_ir::ExprStmt {
                dst: #dst_tokens,
                expr: #expr,
            })
        });
    }
}

impl ToTokens for MacroStmt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            MacroStmt::Barrier { scope } => {
                tokens.extend(quote! {
                    ignis_ir::Stmt::Macro(
                        ignis_ir::MacroStmt::Barrier {
                            scope: #scope,
                        }
                    )
                });
            }
            MacroStmt::MemFence { scope } => {
                tokens.extend(quote! {
                    ignis_ir::Stmt::Macro(
                        ignis_ir::MacroStmt::MemFence {
                            scope: #scope,
                        }
                    )
                });
            }
            MacroStmt::Atomic {
                op,
                buffer,
                index,
                value,
            } => {
                tokens.extend(quote! {
                    ignis_ir::Stmt::Macro(
                        ignis_ir::MacroStmt::Atomic {
                            op: #op,
                            buffer: #buffer,
                            index: #index,
                            value: #value,
                        }
                    )
                });
            }
            MacroStmt::Store {
                buffer,
                index,
                value,
            } => {
                tokens.extend(quote! {
                    ignis_ir::Stmt::Macro(
                        ignis_ir::MacroStmt::Store {
                            buffer: #buffer,
                            index: #index,
                            value: #value,
                        }
                    )
                });
            }
            MacroStmt::If {
                cond,
                then_blk,
                else_blk,
            } => {
                tokens.extend(quote! {
                    ignis_ir::Stmt::Macro(
                        ignis_ir::MacroStmt::If {
                            cond: #cond,
                            then_blk: #then_blk,
                            else_blk: #else_blk,
                        }
                    )
                });
            }
            MacroStmt::For {
                var,
                start,
                end,
                step,
                body,
            } => {
                tokens.extend(quote! {
                    ignis_ir::Stmt::Macro(
                        ignis_ir::MacroStmt::For {
                            var: #var,
                            start: #start,
                            end: #end,
                            step: #step,
                            body: #body,
                        }
                    )
                });
            }
        }
    }
}

impl ToTokens for SSAVar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let v = self.0;
        tokens.extend(quote!(ignis_ir::SSAVar(#v)));
    }
}

impl ToTokens for BufferId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        tokens.extend(quote!(ignis_ir::BufferId(#id)));
    }
}

impl ToTokens for DType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            DType::I32 => quote!(ignis_ir::DType::I32),
            DType::I64 => quote!(ignis_ir::DType::I64),
            DType::U32 => quote!(ignis_ir::DType::U32),
            DType::F16 => quote!(ignis_ir::DType::F16),
            DType::F32 => quote!(ignis_ir::DType::F32),
            DType::F64 => quote!(ignis_ir::DType::F64),
            DType::Bool => quote!(ignis_ir::DType::Bool),
        };
        tokens.extend(q);
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let scalar = &self.scalar;
        let lanes = self.lanes;
        tokens.extend(quote!(ignis_ir::Type { scalar: #scalar, lanes: #lanes }));
    }
}
