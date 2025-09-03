mod error;
mod lowering;
mod parser;

extern crate proc_macro;
use std::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

use crate::parser::Parser;

#[proc_macro_attribute]
pub fn ignis(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let item_fn: ItemFn = parse_macro_input!(input_clone as ItemFn);
    let fn_name = &item_fn.sig.ident;
    let struct_ident = fn_name.clone();

    let mut parser = Parser::new(item_fn.clone());
    let kernel = match parser.parse() {
        Ok(kernel) => kernel,
        Err(_e) => {
            panic!("unable to parse #[ignis");
        }
    };

    let expanded = quote! {
        pub struct #struct_ident;

        impl #struct_ident {
            pub fn to_ir() -> ignis_ir::Kernel {
                #kernel
            }
        }
    };

    expanded.into()
}
