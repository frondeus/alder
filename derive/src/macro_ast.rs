use proc_macro::{TokenStream};
use proc_macro2::{TokenStream as TokenStream2};

use syn::{parse_macro_input, DeriveInput, parse_quote, Ident};
use quote::{quote, ToTokens};
use darling::ast;
use darling::{FromDeriveInput, FromField, FromVariant};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(cst), supports(any))]
struct CstInputReceiver {
    ident: syn::Ident,
    data: ast::Data<CstVariantReceiver, CstFieldReceiver>,
    #[darling(default)]
    parser: Option<syn::Path>,
    #[darling(default)]
    node: Option<syn::Path>,
}

#[derive(Debug, FromField)]
#[darling(attributes(cst))]
struct CstFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    find: Option<syn::Path>,
    #[darling(default)]
    flatten: bool
}

#[derive(Debug, FromVariant)]
#[darling(attributes(cst))]
struct CstVariantReceiver {
    ident: syn::Ident,
    fields: ast::Fields<CstFieldReceiver>,
    #[darling(default)]
    tag: Option<syn::Path>,
    #[darling(default)]
    error: bool,
}


impl ToTokens for CstInputReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let CstInputReceiver {
            ref ident,
            ref data,
            ref parser,
            ref node,
        } = *self;

        let input_ident = ident;
        let mut generated = vec![];

        if let Some(ast::Fields { mut fields, .. }) = data.as_ref().take_struct() {
            let mut idents = vec![];
            let node_field = fields.iter().position(|CstFieldReceiver { ident , ..}| {
                if let Some(ident) = ident {
                    let node_ident: Ident = parse_quote! { node };
                    if ident == &node_ident {
                        return true;
                    }
                }
                else { return true };
                return false;
            })
            .map(|node_field_pos| fields.remove(node_field_pos));

            generated.push(quote! {
                let node = iter.next()?;
            });

            if let Some(node) = node {
                generated.push(quote! {
                    if !node.is(#node) { return None; }
                });
            }

            if let Some(CstFieldReceiver { ident , ..}) = node_field {
                idents.push(ident);
            }

            for CstFieldReceiver { ident, ty, find, flatten, ..} in fields {
                if *flatten {
                    generated.push(quote! {
                        let mut #ident: #ty = Default::default();
                        loop {
                            if let Some(item) = Ast::parse(&mut iter) {
                                pairs.push(item);
                            } else { break; }
                        }
                    });
                }
                else if let Some(find) = find {
                    generated.push(quote! {
                        let #ident = iter.find(|n| n.is(#find))
                        .and_then(|node| {
                            let nodes = node.iter().cloned().collect::<Vec<Node>>();
                            #ty::parse(&mut nodes.into_iter())
                        })?;
                    });
                }
                else {
                    generated.push(quote! {
                        let #ident: #ty = Ast::parse(iter).unwrap();
                    });
                }
                idents.push(ident);
            }
            generated.push(quote! { Some(Self { #(#idents),* }) });
            //dbg!(quote!{#(#generated)*}.to_string());
        }

        if let Some(mut variants) = data.as_ref().take_enum() {

            generated.push(quote! {
                let mut iter = iter.peekable();
                let node = iter.peek()?;
            });

            if let Some(node) = node {
                generated.push(quote! {
                    if !node.is(#node) { return None; }
                });
            } else {
                generated.push(quote!{ if false { unreachable!() } });
            }

            let last = variants.pop();
            for CstVariantReceiver { tag, ident, ..} in variants {
                if let Some(tag) = tag {
                    generated.push(quote! {
                        else if node.is(#tag) {
                            Some(#input_ident::#ident(Ast::parse(&mut iter)?))
                        }
                    });
                }
            }
            if let Some(CstVariantReceiver{ ident, error: true, .. }) = last {
                generated.push(quote! {
                    else {
                        Some(#input_ident::#ident(iter.next().unwrap()))
                    }
                });
            }
        }

        tokens.extend(quote! {
            impl Ast for #ident {
                fn parse(mut iter: &mut impl Iterator<Item = Node>) -> Option<Self> {
                    #(#generated)*
                }
            }
        });

        if let Some(parser) = parser {
            tokens.extend(quote! {
                //impl std::str::FromStr for #ident {
                impl #ident {
                    fn from_str(input: &str) -> Option<Self> {
                        let mut parsed = State::parse(input, #parser());
                        let node = parsed.nodes.pop().unwrap();
                        let nodes = node.iter().cloned().collect::<Vec<Node>>();
                        Self::parse(&mut nodes.into_iter())
                    }
                }
            });
        }
    }
}

fn from_ast(input: &DeriveInput) -> TokenStream2 {
    let cst = match CstInputReceiver::from_derive_input(&input) {
        Ok(c) => c,
        Err(e) => { return e.write_errors(); }
    };
    let tokens = quote!(#cst);
    tokens.into()
}

pub fn ast(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let from_ast = from_ast(&input);

    let res = quote!{
        #from_ast
    };

    res.into()
}
