#![allow(dead_code)]

#[macro_use]
mod macros;

mod core {
    mod input;
    mod offset;
    mod parsed;
    mod parser;
    mod state;

    pub use input::*;
    pub use offset::*;
    pub use parsed::*;
    pub use parser::*;
    pub use state::*;
}

mod cst {
    mod node;
    mod parsers;

    pub use node::*;
    pub use parsers::*;
}

mod ast;

mod display;

pub use crate::core::*;
pub use ast::*;
pub use cst::*;
pub use macros::*;

#[cfg(feature = "derive")]
pub use alder_derive::*;

pub mod testing;
