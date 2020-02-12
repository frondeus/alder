use crate::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NodeId(pub &'static str);

impl NodeId {
    pub const ROOT: Self = NodeId("ROOT");
    pub const TOKEN: Self = NodeId("TOKEN");
    pub const ERROR: Self = NodeId("ERROR");
    pub const EXTRA: Self = NodeId("EXTRA");
    pub const VIRTUAL: Self = NodeId("VIRTUAL");
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub name: NodeId,
    pub alias: Vec<NodeId>,
    pub span: Input,
    pub children: Vec<Node>,
}

impl Node {
    pub fn root(span: Input) -> Self {
        Self {
            name: NodeId::ROOT,
            span,
            children: vec![],
            alias: vec![],
        }
    }

    pub fn token(span: Input) -> Self {
        Self {
            name: NodeId::TOKEN,
            span,
            children: vec![],
            alias: vec![],
        }
    }

    pub fn error(span: Input) -> Self {
        Self {
            name: NodeId::ERROR,
            span,
            children: vec![],
            alias: vec![],
        }
    }

    //#[cfg(test)] TODO
    pub fn test(name: NodeId, input: &str, range: (usize, usize), alias: &[NodeId]) -> Self {
        let span = Input::new_test(input, range);
        let mut node = Self {
            name,
            span,
            alias: vec![],
            children: vec![],
        };
        node.alias.extend(alias);
        node
    }

    pub fn is(&self, name: NodeId) -> bool {
        self.name == name || self.alias.iter().any(|alias| *alias == name)
    }
}
