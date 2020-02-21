use crate::test_case::TestCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;
use syn::{parse_macro_input, AttrStyle, ItemFn, Lit, Meta};
use unindent::Unindent;

pub fn alder_test(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);

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
