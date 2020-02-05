use crate::*;

#[derive(Debug)]
pub struct Parsed<'a, T> {
    pub input: Input<'a>,
    pub root: T,
    pub rest: Rest<'a>,
    pub problems: Vec<(Box<dyn Problem + 'a>, Location<'a>)>,
}

impl<'a, T: Display> Display for Parsed<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "```")?;
        writeln!(f, "{}", self.input)?;
        writeln!(f, "```")?;
        writeln!(f)?;
        writeln!(f, "{}", self.root)?;
        if !self.rest.is_empty() {
            writeln!(f, "REST: {:?}", self.rest)?;
        }
        if self.problems.is_empty() {
            writeln!(f, "NO PROBLEMS")?;
        } else {
            writeln!(f, "PROBLEMS:")?;
            for (p, loc) in self.problems.iter() {
                writeln!(f, "* {} - {:?}", p, loc)?;
            }
        }
        Ok(())
    }
}

mod node;
mod node_kind;
mod node_vec;

pub use self::node::*;
pub use self::node_kind::*;
pub use self::node_vec::*;
use std::fmt::{Display, Error, Formatter};
