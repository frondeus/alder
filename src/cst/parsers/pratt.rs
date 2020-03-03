use crate::*;

/*
pub fn pratt(
    name: NodeId,
    alias: Vec<NodeId>,
    next: impl Clone + Parser,
    bp: impl Clone + for<'a> Fn(&'a str) -> Option<(i32, &'static str)>,
) -> impl Parser {
    infix(name, alias, 0, next, bp)
}
*/
pub fn pratt(
    name: NodeId,
    alias: Vec<NodeId>,
    next: impl Clone + Parser,
    bp: impl Clone + Fn(&mut State) -> Option<(i32, Box<dyn Parser>)>,
) -> impl Parser {
    infix(name, alias, 0, next, bp)
}

pub fn infix(
    name: NodeId,
    alias: Vec<NodeId>,
    rbp: i32,
    next: impl Clone + Parser,
    bp: impl Clone + Fn(&mut State) -> Option<(i32, Box<dyn Parser>)>,
) -> impl Parser {
    move |state: &mut State| {
        let mut left = next.parse(state);
        if left.is(NodeId::ERROR) || left.has(NodeId::ERROR) {
            return left;
        }
        loop {
            let span = state.input.clone();
            let (op_bp, op_token) = match bp(state) {
                Some(op) if op.0 > rbp => op,
                _ => return left,
            };

            let res = Node {
                name,
                span,
                alias: alias.clone(),
                children: vec![],
            };
            state.nodes.push(res);

            state.add_node(left);
            state.add(|state: &mut State| op_token.parse(state));
            state.add(infix(
                name,
                alias.clone(),
                op_bp - 1,
                next.clone(),
                bp.clone(),
            ));

            left = state.nodes.pop().expect("Node");
            left.recalc_span(state);
        }
    }
}
