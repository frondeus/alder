extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::{parse_macro_input, AttrStyle, ItemFn, Lit, Meta};
use unindent::Unindent;

use quote::quote;
use syn::parse_quote;

#[derive(Debug)]
struct TestCase {
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
        let name = escape_test_name(format!("{}_{}_{}", self.fn_name, i, input));

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

#[proc_macro_attribute]
pub fn alder(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as ItemFn);

    let fn_name = item.sig.ident.to_string();

    let test_cases = item
        .attrs
        .iter()
        .filter(|attr| {
            let style = match attr.style {
                AttrStyle::Outer => true,
                _ => false,
            };

            let is_doc = { attr.path == parse_quote!(doc) };

            style && is_doc
        })
        .filter_map(|a| a.parse_meta().ok())
        .filter_map(|meta| match meta {
            Meta::NameValue(nv) => Some(nv.lit),
            _ => None,
        })
        .filter_map(|s| match s {
            Lit::Str(s) => Some(s.value()),
            _ => None,
        })
        .map(|lit| lit.unindent())
        .map(|lit| lit.trim_start().to_string())
        .map(|lit| TestCase::new(fn_name.clone(), lit))
        .collect::<Vec<_>>();

    let test_cases = test_cases
        .into_iter()
        .enumerate()
        .map(|(i, ts)| ts.render(i))
        .collect::<Vec<_>>();

    let out = quote! {
        #item

        #(#test_cases)*
    };

    out.into()
}

fn escape_test_name(input: impl AsRef<str>) -> Ident {
    if input.as_ref().is_empty() {
        return Ident::new("_empty", Span::call_site());
    }

    let mut last_under = false;
    let mut ident: String = input
        .as_ref()
        .to_ascii_lowercase()
        .chars()
        .filter_map(|c| match c {
            c if c.is_alphanumeric() => {
                last_under = false;
                Some(c.to_ascii_lowercase())
            }
            _ if !last_under => {
                last_under = true;
                Some('_')
            }
            _ => None,
        })
        .collect();

    if !ident.starts_with(|c: char| c == '_' || c.is_ascii_alphabetic()) {
        ident = format!("_{}", ident);
    }
    Ident::new(&ident, Span::call_site())
}
