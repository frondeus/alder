use proc_macro2::{Ident, Span};

pub fn escape_test_name(input: impl AsRef<str>) -> Ident {
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
