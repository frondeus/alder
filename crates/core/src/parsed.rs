use crate::*;
use std::fmt::{Debug, Display};

pub trait Problem: Debug + Display {}

impl<P> Problem for P where P: Debug + Display {}

#[derive(Debug)]
pub struct Parsed {
    pub input: Input,
    pub rest: Input,
    pub nodes: Vec<Node>,
    pub problems: Vec<(Box<dyn Problem>, Input)>,
}
