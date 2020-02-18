use crate::*;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub &'static str);

impl NodeId {
    pub const ROOT: Self = NodeId("ROOT");
    pub const TOKEN: Self = NodeId("TOKEN");
    pub const ERROR: Self = NodeId("ERROR");
    pub const EXTRA: Self = NodeId("EXTRA");
    pub const VIRTUAL: Self = NodeId("VIRTUAL");
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Node {
    pub name: NodeId,
    pub alias: Vec<NodeId>,
    pub span: Input,
    pub children: Vec<Node>,
}

impl Node {
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        std::iter::once(self)
            .chain(self.children.iter())
    }
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

    pub fn add_alias(&mut self, alias: NodeId) {
        if !self.is(alias) {
            self.alias.push(alias);
        }
    }

    pub fn add_aliases(&mut self, aliases: &[NodeId]) {
        for alias in aliases { self.add_alias(*alias); }
    }

    pub fn is(&self, name: NodeId) -> bool {
        self.name == name || self.alias.iter().any(|alias| *alias == name)
    }
}
