use crate::*;

mod common;
mod extra;
mod lexer;
mod node;
mod problem;

pub use common::*;
pub use extra::*;
pub use lexer::utf::*;
pub use lexer::*;
pub use node::*;
pub use problem::*;

use std::marker::PhantomData;
pub struct Map<P, F, T>
where
    P: Parser<T>,
{
    parser: P,
    f: F,
    _phantom: PhantomData<T>,
}

impl<P, F, T> Map<P, F, T>
where
    P: Parser<T>,
{
    pub fn new(parser: P, f: F) -> Self {
        Self {
            parser,
            f,
            _phantom: PhantomData,
        }
    }
}
impl<T, O, P, F> Parser<O> for Map<P, F, T>
where
    P: Parser<T>,
    F: Fn(T) -> O,
{
    fn parse(&self, state: &mut State) -> O {
        let node = self.parser.parse(state);
        (self.f)(node)
    }
}

pub fn map<T, O>(parser: impl Parser<T>, f: impl Fn(T) -> O) -> impl Parser<O> {
    Map::new(parser, f)
}

pub struct AsExtra<P>(P);

impl<P> Parser<Node> for AsExtra<P>
where
    P: Parser<Node>,
{
    fn parse(&self, state: &mut State) -> Node {
        let node = self.0.parse(state);
        node.with_alias(NodeId::EXTRA)
    }
}

pub trait NodeParserExt: Parser<Node> {
    fn as_extra(self) -> AsExtra<Self>
    where
        Self: Sized,
    {
        AsExtra(self)
    }
}

impl<P> NodeParserExt for P where P: Parser<Node> {}
