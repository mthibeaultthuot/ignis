mod error;
mod parser;
mod stmt;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn ignis(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let func: ItemFn = parse_macro_input!(input_clone as ItemFn);
    let parser = parser::Parser::from_item(func);
    parser.parse();
    input
}
