#![allow(dead_code)]

#[macro_use]
mod macros;

mod core {
    mod offset;
    mod parsed;
    mod parser;
    mod span;
    mod state;

    pub use offset::*;
    pub use parsed::*;
    pub use parser::*;
    pub use span::*;
    pub use state::*;
}

mod cst {
    mod node;
    mod parsers;

    pub use node::*;
    pub use parsers::*;
}

mod ast;

#[cfg(feature = "tty")]
mod display;

pub use crate::core::*;
pub use ast::*;
pub use cst::*;
pub use macros::*;

#[cfg(feature = "derive")]
pub use alder_derive::*;

pub mod testing;
