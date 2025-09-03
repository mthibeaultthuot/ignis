# Ignis
##### Rust-first experimental deep learning compiler & kernel DSL

___

## Status
##### ⚠️ Work in progress — very early ⚠️


___

## What it does today
- Provide a proc-macro for parsing syn ItemFn
- Lower to simple binary expressions (a + b)


example:
```rust
#[ignis]
fn add(a: f32, b: f32) -> f32 {
    a + b
}
```

exemple output :
```
Kernel {
  block: KernelBlock {
    stmts: [
      Expr(
        ExprStmt {
          dst: Some(%2),
          expr: Binary { op: Add, lhs: %0, rhs: %1, ty: F32 }
        }
      )
    ]
  }
}
```
___


### Why?

Ignis is a personal learning project:

* to understand how deep learning frameworks work under the hood,
* to practice Rust macros, IR design, and compiler passes,
* and maybe one day turn into a usable research tool.
