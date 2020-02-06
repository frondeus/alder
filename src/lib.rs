use std::fmt::Debug;

mod typedefs {
    pub type Location<'a> = &'a str;
    pub type Input<'a> = &'a str;
    pub type Rest<'a> = &'a str;
}

#[macro_use]
mod macros;
mod offset;
mod parsed;
mod state;
pub mod testing;

pub mod lexer;
pub mod parser;

#[cfg(feature = "derive")]
pub use alder_derive::*;
pub use macros::*;
pub use offset::*;
pub use parsed::*;
pub use state::*;
pub use typedefs::*;

pub trait Parser<'a> {
    fn parse_state(&self, state: State<'a>) -> State<'a>;

    fn parse(&self, input: Input<'a>) -> Parsed<'a> {
        let state = input.into();
        let mut state = self.parse_state(state);
        let rest = state.input;
        let root = state.nodes.pop().unwrap();

        Parsed {
            input,
            root,
            rest,
            problems: state.into(),
        }
    }
}

/// Implementation for every closure.
impl<'a, F> Parser<'a> for F
where
    F: Fn(State<'a>) -> State<'a>,
{
    fn parse_state(&self, state: State<'a>) -> State<'a> {
        self(state)
    }
}
