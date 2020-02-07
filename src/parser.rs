use crate::*;

pub fn node<'a, F>(kind: NodeKind, f: F) -> impl Parser<'a>
where
    F: Fn(State<'a>) -> State<'a>,
{
    move |mut state: State<'a>| {
        let n = Node {
            kind,
            aliases: vec![],
            location: state.input,
            children: vec![],
            expect_children: true,
        };
        state.nodes.push(n);
        let mut state = f(state);
        if let Some(mut n) = state.nodes.pop() {
            let rest = state.input;
            let index = n.location.offset(rest);
            n.location = &n.location[..index];
            n.expect_children = false;
            state.add(n);
        }
        state
    }
}

pub fn v_node<'a>(f: impl Fn(State<'a>) -> State<'a>) -> impl Parser<'a> {
    move |vstate: State<'a>| f(vstate)
}

pub fn alias<'a>(kind: NodeKind, p1: impl Parser<'a>) -> impl Parser<'a> {
    move |mut state: State<'a>| {
        state.push_alias(kind);
        let mut state = p1.parse_state(state);
        state.pop_alias();
        state
    }
}

pub fn repeat<'a>(f: impl Fn(State<'a>, &mut bool) -> State<'a>) -> impl Parser<'a> {
    move |mut state: State<'a>| {
        let mut end = false;
        loop {
            state = f(state, &mut end);
            if end {
                return state;
            }
        }
    }
}
