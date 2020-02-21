extern crate proc_macro;

use proc_macro::TokenStream;

mod macro_alder;
mod macro_ast;
mod test_case;
mod utils;

#[proc_macro_attribute]
pub fn alder_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    macro_alder::alder_test(input)
}

#[proc_macro_derive(Ast, attributes(cst))]
pub fn ast(input: TokenStream) -> TokenStream {
    macro_ast::ast(input)
}
