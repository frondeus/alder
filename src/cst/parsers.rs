use crate::*;

mod extra;
mod lexer;
mod node;
mod problem;

pub use extra::*;
pub use lexer::*;
pub use node::*;
pub use problem::*;

pub fn map<T, O>(parser: impl Parser<T>, f: impl Fn(T) -> O) -> impl Parser<O> {
    move |state: &mut State| {
        let node = parser.parse(state);
        f(node)
    }
}
