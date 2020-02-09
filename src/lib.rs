#![allow(dead_code)]

mod input;
mod node;
mod offset;
mod parser;
mod parsers;
mod parsed;
mod state;
mod display;
#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! node_ids {
        ($name: ident: $first_kind: ident, $($kind: ident),*) => {
            struct $name;
            #[allow(non_upper_case_globals)]
            impl $name {
                const $first_kind: NodeId = NodeId(stringify!($first_kind));
                $( const $kind: NodeId = NodeId(stringify!($kind)); )*
            }
        };
    }
}

pub mod testing;

pub use input::*;
pub use node::*;
pub use offset::*;
pub use parser::*;
pub use parsers::*;
pub use parsed::*;
pub use state::*;
pub use macros::*;
