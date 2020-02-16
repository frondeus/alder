use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use crate::utils::escape_test_name;

#[derive(Debug)]
pub struct TestCase {
    name: String,
    fn_name: String,
    input: String,
}

impl TestCase {
    pub fn new(fn_name: String, input: String) -> Self {
        let name = String::default();

        Self {
            name,
            fn_name,
            input,
        }
    }

    pub fn render(&self, i: usize) -> TokenStream2 {
        let fn_name = Ident::new(&self.fn_name, Span::call_site());
        let input = &self.input;
        let name = escape_test_name(format!("{}_{}", self.fn_name, i));

        quote! {
            #[cfg(test)]
            #[test]
            fn #name() {
                let input = #input;
                let actual = alder::State::parse(input, #fn_name());
                let actual_dbg = format!("{}", actual);
                alder::testing::snap(actual_dbg, module_path!(), stringify!(#name));
            }
        }
    }
}
