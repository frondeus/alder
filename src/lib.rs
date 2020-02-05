use std::fmt::Debug;

mod typedefs {
    pub type Location<'a> = &'a str;
    pub type Input<'a> = &'a str;
    pub type Rest<'a> = &'a str;
}

#[macro_use]
mod macros;
mod combinators;
mod offset;
mod parsed;
mod state;
pub mod testing;

pub mod lexer;
pub mod parser;

#[cfg(feature = "derive")]
pub use alder_derive::*;
pub use combinators::*;
pub use macros::*;
pub use offset::*;
pub use parsed::*;
pub use state::*;
pub use typedefs::*;

pub trait Parser<'a> {
    type Output;

    fn parse_state(&self, i: Input<'a>, state: &mut State<'a>) -> (Self::Output, Rest<'a>);

    fn parse(&self, i: Input<'a>) -> Parsed<'a, Self::Output> {
        let mut state = State::default();
        let (root, rest) = self.parse_state(i, &mut state);

        Parsed {
            input: i,
            root,
            rest,
            problems: state.into(),
        }
    }

    // Combinators
    fn map<Output2, Func>(self, f: Func) -> Map<Self, Func>
    where
        Func: Fn(Self::Output) -> Output2,
        Self: Sized,
    {
        Map { p: self, f }
    }
}

/// Implementation for every closure.
impl<'a, Output, F> Parser<'a> for F
where
    F: Fn(Input<'a>, &mut State<'a>) -> (Output, Rest<'a>),
{
    type Output = Output;

    fn parse_state(&self, i: Input<'a>, state: &mut State<'a>) -> (Self::Output, Rest<'a>) {
        self(i, state)
    }
}
