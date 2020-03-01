use crate::*;

pub fn infix(
    name: NodeId,
    rbp: i32,
    next: impl Parser + Clone,
    bp: impl Clone + for<'a> Fn(&'a str) -> Option<(i32, &'static str)>,
) -> impl Parser {
    move |state: &mut State| {
        let mut left = next.parse(state);
        loop {
            let op = state.peek(1);
            let (op_bp, op_token) = match bp(op.as_ref()) {
                Some(op) if op.0 > rbp => op,
                _ => return left,
            };

            let res = Node {
                name,
                span: state.input.clone(),
                alias: vec![],
                children: vec![],
            };
            state.nodes.push(res);

            state.add_node(left);
            state.add(token(op_token));
            state.add(infix(name, op_bp - 1, next.clone(), bp.clone()));

            left = state.nodes.pop().expect("Node");
            left.recalc_span(state);
        }
    }
}
