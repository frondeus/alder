use crate::*;
use derive_more::Display;

pub fn map(parser: impl Parser, f: impl Fn(Node) -> Node) -> impl Parser {
    move |state: &mut State| {
        let node = parser.parse(state);
        f(node)
    }
}

#[derive(Debug, Display, Clone)]
enum LexerError {
    #[display(fmt = "I expected `{}`", _0)]
    ExpectedTag(&'static str),

    #[display(fmt = "I expected `{}`", _0)]
    UnexpectedEOF(&'static str),
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

pub fn recover(parser: impl Parser) -> impl Parser {
    move |state: &mut State| {
        let node = parser.parse(state);
        if !node.is(NodeId::ERROR) {
            state.panic = false;
        }
        node
    }
}

impl Parser for &'static str {
    fn parse(&self, state: &mut State) -> Node {
        token(self).parse(state)
    }
}

pub fn token(token: &'static str) -> impl Parser {
    move |state: &mut State| {
        let size = token.len();
        let i_size = state.input.len();
        if i_size >= size {
            let t = state.input.peek_str(size);
            if t == token {
                Node::token(state.input.chomp(size))
            } else {
                raise(LexerError::ExpectedTag(token), size).parse(state)
            }
        } else {
            raise(LexerError::UnexpectedEOF(token), i_size).parse(state)
        }
    }
}

pub fn chomp_while(name: NodeId, f: impl Fn(char) -> bool) -> impl Parser {
    move |state: &mut State| {
        let mut len = 0;
        loop {
            let c = &state.input.as_ref()[len..].chars().next();
            match c {
                Some(letter) if f(*letter) => {
                    len += letter.len_utf8();
                }
                _ => break,
            }
        }

        if len > 0 {
            let result = state.input.chomp(len);

            Node {
                name,
                span: result,
                children: vec![],
                alias: vec![],
            }
        } else {
            none().parse(state)
        }
    }
}

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

pub struct WithExtra<P: Parser> {
    extra: std::sync::Arc<dyn Parser>,
    parser: P,
}

impl<P: Parser> Parser for WithExtra<P> {
    fn parse(&self, state: &mut State) -> Node {
        let extra = self.extra.clone();
        state.push_extra(extra);
        let node = self.parser.parse(state);
        state.pop_extra();
        node
    }
}

pub fn with_extra<P: Parser>(extra: std::sync::Arc<dyn Parser>, parser: P) -> WithExtra<P> {
    WithExtra { extra, parser }
}

pub fn no_extra(parser: impl Parser) -> impl Parser {
    move |state: &mut State| {
        state.push_atomic();
        let node = parser.parse(state);
        state.pop_extra();
        node
    }
}

pub fn field(name: NodeId, f: impl Parser) -> impl Parser {
    map(f, move |mut node| {
        node.alias.push(name);
        node
    })
}
