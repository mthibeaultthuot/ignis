mod error;
mod lowering;
mod parser;

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

use crate::parser::Parser;

#[proc_macro_attribute]
pub fn ignis(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let item_fn: ItemFn = parse_macro_input!(input_clone as ItemFn);
    let mut parser = Parser::new(item_fn);
    let result = parser.parse();
    match result {
        Ok(kernel) => {
            println!("{:#?}", kernel);
        }
        Err(e) => {
            panic!("ignis macro failed: {}", e);
        }
    }

    input
}
