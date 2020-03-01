use crate::*;

pub fn none() -> impl Parser {
    node(NodeId::VIRTUAL, |_| ())
}

pub fn v_node(name: impl Into<Option<NodeId>>, f: impl Fn(&mut State)) -> impl Parser {
    let alias = name.into().into_iter().collect();
    node_inner(NodeId::VIRTUAL, alias, f)
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
        let mut res = state.nodes.pop().expect("Node");
        res.recalc_span(state);
        res
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
