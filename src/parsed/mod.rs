use crate::*;

#[derive(Debug)]
pub struct Parsed<'a> {
    pub input: Input<'a>,
    pub roots: Vec<Node<'a>>,
    pub rest: Rest<'a>,
    pub problems: Vec<(Box<dyn Problem + 'a>, Location<'a>)>,
}

impl<'a> Display for Parsed<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "```")?;
        writeln!(f, "{}", self.input)?;
        writeln!(f, "```")?;
        writeln!(f)?;
        for root in self.roots.iter() {
            write!(f, "{}", root)?;
        }
        writeln!(f)?;
        if !self.rest.is_empty() {
            writeln!(f, "REST: {:?}", self.rest)?;
        }
        if self.problems.is_empty() {
            writeln!(f, "NO PROBLEMS")?;
        } else {
            writeln!(f, "PROBLEMS:")?;
            for (p, loc) in self.problems.iter() {
                writeln!(f, "* {}", p)?;
                writeln!(f, "```")?;
                writeln!(f, "{}", loc)?;
                writeln!(f, "```")?;
            }
        }
        Ok(())
    }
}

mod node;
mod node_kind;

pub use self::node::*;
pub use self::node_kind::*;
use std::fmt::{Display, Error, Formatter};
