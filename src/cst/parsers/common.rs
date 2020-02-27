use crate::*;

pub fn is_ws(s: &str) -> bool {
    match s {
        " " | "\t" | "\r\n" | "\n" => true,
        _ => false,
    }
}

pub fn is_inline_ws(s: &str) -> bool {
    match s {
        " " | "\t" => true,
        _ => false,
    }
}

pub fn is_line_ending(s: &str) -> bool {
    match s {
        "\r\n" | "\n" => true,
        _ => false,
    }
}

pub fn is_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn ws0() -> impl Parser<Input> {
    chomp_while(is_ws)
}
pub fn inline_ws0() -> impl Parser<Input> {
    chomp_while(is_inline_ws)
}
pub fn line_ending0() -> impl Parser<Input> {
    chomp_while(is_line_ending)
}

pub trait UtfExt {
    fn is_ws(&self) -> bool;
    fn is_inline_ws(&self) -> bool;
    fn is_line_ending(&self) -> bool;
    fn is_hex(&self) -> bool;
}

impl<'a> UtfExt for &'a str {
    fn is_ws(&self) -> bool {
        is_ws(self)
    }

    fn is_inline_ws(&self) -> bool {
        is_inline_ws(self)
    }

    fn is_line_ending(&self) -> bool {
        is_line_ending(self)
    }

    fn is_hex(&self) -> bool {
        is_hex(self)
    }
}
