mod builtins;
mod context;
mod error;
mod expr;
mod kernel;
mod memory;
mod ops;
mod parser;
mod stmt;
mod to_token_impl;
mod type_sys;

use builtins::*;
use context::*;
use expr::*;
use kernel::*;
use memory::*;
use ops::*;
use stmt::*;
use type_sys::*;

extern crate proc_macro;
use std::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

use crate::parser::Parser;

#[proc_macro_attribute]
pub fn ignis(_attr: TokenStream, input: TokenStream) -> TokenStream {
    match ignis_impl(input) {
        Ok(token) => token,
        Err(_) => panic!("unable to parse #[ignis]"),
    }
}

fn ignis_impl(input: TokenStream) -> Result<proc_macro::TokenStream, syn::Error> {
    let item_fn: ItemFn = syn::parse(input)?;
    let fn_name = &item_fn.sig.ident;
    let struct_ident = fn_name.clone();

    let mut parser = Parser::new(item_fn.clone());
    let kernel = match parser.parse() {
        Ok(kernel) => kernel,
        Err(_) => panic!("unable to parse #[ignis"),
    };

    let expanded = quote! {
        pub struct #struct_ident;
        impl #struct_ident {
            pub fn to_ir() -> Kernel {
                #kernel
            }
        }
    };

    Ok(expanded.into())
}
