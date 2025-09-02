mod error;
mod parser;

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ignis(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
