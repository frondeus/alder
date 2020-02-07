use crate::*;
use std::fmt::Display;

pub trait Problem: Display + Debug {}

impl<E> Problem for E where E: Display + Debug {}

#[derive(Debug)]
pub struct State<'a> {
    pub(crate) input: Input<'a>,
    pub(crate) nodes: Vec<Node<'a>>,
    problems: Vec<(Box<dyn Problem + 'a>, Location<'a>)>,
    panic: bool,
}

impl<'a> State<'a> {
    pub fn raise<P: 'a + Problem>(mut self, problem: P, loc: Location<'a>) -> Self {
        self.problems.push((Box::new(problem), loc));
        self.panic = true;
        self.add(Node::error(loc));
        self
    }

    pub fn fuse(&mut self) {
        self.panic = false;
    }

    fn node(&mut self) -> Option<&mut Node<'a>> {
        self.nodes.last_mut()
    }

    pub fn chomp(&mut self, len: usize) -> &'a str {
        let chomped = &self.input[..len];
        self.input = &self.input[len..];
        chomped
    }

    pub fn consume(mut self, len: usize) -> Self {
        self.chomp(len);
        self
    }

    pub fn add(&mut self, node: Node<'a>) {
        if let Some(r @ Node { expect_children: true, .. }) = self.node() {
            r.children.push(node);
        } else {
            self.nodes.push(node);
        }
    }

    pub fn peek(self, mut f: impl FnMut(Option<char>, Self) -> Self) -> Self {
        let c = self.input.chars().next();
        f(c, self)
    }

    pub fn peek_nth(self, nth: usize, mut f: impl FnMut(Option<char>, Self) -> Self) -> Self {
        let c = self.input.chars().nth(nth);
        f(c, self)
    }

    pub fn skip(self) -> Self {
        self
    }

    pub fn parse(self, parser: impl Parser<'a>) -> Self {
        parser.parse_state(self)
    }
}

impl<'a> From<Input<'a>> for State<'a> {
    fn from(input: Input<'a>) -> Self {
        State {
            input,
            panic: false,
            problems: vec![],
            nodes: vec![],
        }
    }
}

impl<'a> Into<Vec<(Box<dyn Problem + 'a>, Location<'a>)>> for State<'a> {
    fn into(self) -> Vec<(Box<dyn Problem + 'a>, Location<'a>)> {
        self.problems
    }
}
