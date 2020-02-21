use crate::*;

pub fn none() -> impl Parser {
    node(NodeId::VIRTUAL, |_| ())
}

pub fn v_node(name: NodeId, f: impl Fn(&mut State)) -> impl Parser {
    node_inner(NodeId::VIRTUAL, vec![name], f)
}

fn node_inner(name: NodeId, alias: Vec<NodeId>, f: impl Fn(&mut State)) -> impl Parser {
    move |state: &mut State| {
        let n = Node {
            name,
            span: state.input.clone(),
            alias: alias.clone(),
            children: vec![],
        };
        state.nodes.push(n);
        f(state);
        let mut n = state.nodes.pop().expect("Node");
        let rest = &state.input;
        let index = n.span.offset(rest);
        let len = index;
        n.span = n.span.chomp(len);
        n
    }
}

pub fn node(name: NodeId, f: impl Fn(&mut State)) -> impl Parser {
    node_inner(name, vec![], f)
}

pub fn field(name: NodeId, f: impl Parser) -> impl Parser {
    map(f, move |mut node| {
        node.alias.push(name);
        node
    })
}
