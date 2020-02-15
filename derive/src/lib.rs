#![feature(drain_filter)]
extern crate proc_macro;

use proc_macro::TokenStream;

mod test_case;
mod utils;
mod macro_alder;
mod macro_ast;

#[proc_macro_attribute]
pub fn alder(_args: TokenStream, input: TokenStream) -> TokenStream {
    macro_alder::alder(input)
}

#[proc_macro_derive(Ast, attributes(cst))]
pub fn ast(input: TokenStream) -> TokenStream {
    macro_ast::ast(input)
}

