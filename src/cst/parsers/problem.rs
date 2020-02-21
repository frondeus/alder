use crate::*;

pub fn recover(parser: impl Parser) -> impl Parser {
    move |state: &mut State| {
        let node = parser.parse(state);
        if !node.is(NodeId::ERROR) {
            state.panic = false;
        }
        node
    }
}

pub fn raise(problem: impl Problem + Clone + 'static, len: usize) -> impl Parser {
    move |state: &mut State| {
        let panic = state.panic;
        let span = state.input.chomp(len);
        match state.last_error() {
            Some(err) if panic => {
                err.span.range.1 += len;
                if let Some(error) = state.errors.last_mut() {
                    error.span.range.1 += len;
                }
                none().parse(state)
            }
            _ if !panic => {
                let problem = Box::new(problem.clone()) as Box<dyn Problem + 'static>;
                let context = state
                    .nodes
                    .iter()
                    .flat_map(|node| node.all_names_with_span())
                    .filter(|(name, _)| !NodeId::NO_CONTEXT.contains(name))
                    .map(|(name, span)| ParseErrorContext::new(name, span))
                    .collect();
                state
                    .errors
                    .push(ParseError::new(problem, span.clone(), context));
                state.panic = true;
                Node::error(span)
            }
            _ => Node::error(span),
        }
    }
}
