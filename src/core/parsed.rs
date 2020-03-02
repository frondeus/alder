use crate::*;
use std::fmt::{Debug, Display};

pub trait Problem: Debug + Display {}

impl<P> Problem for P where P: Debug + Display {}

#[derive(Debug)]
pub struct Parsed {
    pub input: Span,
    pub rest: Span,
    pub nodes: Vec<Node>,
    pub errors: Vec<ParseError>,
}
