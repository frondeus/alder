use crate::*;

pub fn node<'a, F>(kind: NodeKind, f: F) -> impl Parser<'a>
where
    F: Fn(State<'a>) -> State<'a>,
{
    move |mut state: State<'a>| {
        let n = Node {
            kind,
            location: state.input,
            children: vec![],
        };
        state.nodes.push(n);
        let mut state = f(state);
        if let Some(mut n) = state.nodes.pop() {
            let rest = state.input;
            let index = n.location.offset(rest);
            n.location = &n.location[..index];
            state.add(n);
        }
        state
    }
}

pub fn v_node<'a>(f: impl Fn(State<'a>) -> State<'a>) -> impl Parser<'a> {
    move |vstate: State<'a>| f(vstate)
}
