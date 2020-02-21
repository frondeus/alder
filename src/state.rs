use crate::*;
use std::fmt::{Debug, Error, Formatter};
use std::sync::Arc;

#[derive(Debug)]
pub struct ParseErrorContext {
    pub node: NodeId,
    pub span: Input,
}

impl ParseErrorContext {
    pub fn new(node: NodeId, span: Input) -> Self {
        Self { node, span }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub problem: Box<dyn Problem + 'static>,
    pub span: Input,
    pub context: Vec<ParseErrorContext>,
}

impl ParseError {
    pub fn new(
        problem: Box<dyn Problem + 'static>,
        span: Input,
        context: Vec<ParseErrorContext>,
    ) -> Self {
        Self {
            problem,
            span,
            context,
        }
    }
}

pub struct State {
    pub input: Input,
    pub nodes: Vec<Node>,
    extras: Vec<Option<Arc<dyn Parser>>>,
    parsing_extra: bool,
    pub(crate) errors: Vec<ParseError>,
    pub panic: bool,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.input.fmt(f)?;
        self.nodes.fmt(f)?;
        Ok(())
    }
}

impl<'a> From<&'a str> for State {
    fn from(input: &'a str) -> State {
        let input: Input = input.into();
        Self {
            input: input.clone(),
            nodes: vec![Node::root(input)],
            extras: vec![],
            errors: vec![],
            parsing_extra: false,
            panic: false,
        }
    }
}

impl State {
    pub fn parse(input: &str, parser: impl Parser) -> Parsed {
        let mut state = Self::from(input);
        state.add(parser);
        let nodes = state.nodes.pop().expect("At least root").children;
        Parsed {
            input: input.into(),
            rest: state.input,
            nodes,
            errors: state.errors,
        }
    }

    pub fn add(&mut self, parser: impl Parser) {
        if !self.parsing_extra && !self.panic {
            self.add_extra();
        }

        let node = parser.parse(self);
        self.add_node(node);

        if !self.parsing_extra && !self.panic {
            self.add_extra();
        }
    }
}

impl State {
    pub(crate) fn push_extra(&mut self, extra: std::sync::Arc<dyn Parser>) {
        self.extras.push(Some(extra));
    }

    pub(crate) fn push_atomic(&mut self) {
        self.extras.push(None);
    }

    pub(crate) fn pop_extra(&mut self) {
        self.extras.pop();
    }

    pub(crate) fn add_node(&mut self, node: Node) {
        let parent = self.node().expect("At least root");

        if node.is(NodeId::VIRTUAL) {
            for mut child in node.children {
                if !child.is(NodeId::EXTRA) {
                    child.add_aliases(node.alias.as_slice());
                }
                parent.children.push(child);
            }
            return;
        }

        parent.children.push(node);
    }

    pub(crate) fn last_error(&mut self) -> Option<&mut Node> {
        self.node()
            .and_then(|root| root.children.last_mut())
            .and_then(|node| {
                if node.is(NodeId::ERROR) {
                    Some(node)
                } else {
                    None
                }
            })
    }

    pub(crate) fn pop_node(&mut self) -> Option<Node> {
        self.node().and_then(|root| root.children.pop())
    }
}

impl State {
    fn node(&mut self) -> Option<&mut Node> {
        self.nodes.last_mut()
    }

    fn add_extra(&mut self) {
        if let Some(Some(extra)) = self.extras.last() {
            self.parsing_extra = true;
            let extra = extra.clone();
            let mut extra_node = extra.parse(self);
            extra_node.add_alias(NodeId::EXTRA);
            self.add_node(extra_node);
            self.parsing_extra = false;
        }
    }
}
