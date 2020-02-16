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
    #[display(fmt = "Expected `{}`, but found:", _0)]
    ExpectedTag(&'static str),

    #[display(fmt = "Expected `{}`, but found:", _0)]
    ExpectedChar(char),

    #[display(fmt = "Expected `{}`, but found:", _0)]
    UnexpectedEOF(&'static str),

    #[display(fmt = "Expected `{}`, but found:", _0)]
    UnexpectedEOFChar(char)
}

pub fn raise(problem: impl Problem  + Clone + 'static, len: usize) -> impl Parser {
    move |state: &mut State| {
        let panic = state.panic;
        let span = state.input.chomp(len);
        dbg!(&panic);
        dbg!(&span);
        dbg!(&state);
        match dbg!(state.last_error()) {
            Some(_) if panic => {
                let mut err = state.pop_node().unwrap(); //Unwrap: Some(_)
                dbg!(&err);
                err.span.range.1 = span.range.0;
                err
            },
            _ if !panic => {
                let problem = Box::new(problem.clone()) as Box<dyn Problem + 'static>;
                state.problems.push((problem, span.clone()));
                state.panic = true;
                Node::error(span)
            },
            _ => Node::error(span)
        }
    }
}

pub fn fuse(parser: impl Parser) -> impl Parser {
    move |state: &mut State| {
        let node = parser.parse(state);
        if !node.is(NodeId::ERROR) { state.panic = false; }
        node
    }
}

pub fn tag(tag: &'static str) -> impl Parser {
    move |state: &mut State| {
        let size = tag.len();
        let i_size = state.input.len();
        if i_size >= size {
            let t = state.input.peek_str(size);
            if t == tag {
                Node::token(state.input.chomp(size))
            }
            else {
                raise(LexerError::ExpectedTag(tag), size).parse(state)
            }
        }
        else {
            raise(LexerError::UnexpectedEOF(tag), i_size).parse(state)
        }
    }
}

pub fn token(token: char) -> impl Parser {
    move |state: &mut State| {
        let next = state.input.as_ref().chars().next();
        match next {
            Some(n) if n == token => Node::token(state.input.chomp(1)),
            Some(_) =>
                raise(LexerError::ExpectedChar(token), 1).parse(state),
                //Node::error(state.input.chomp(1)),
            None => raise(LexerError::UnexpectedEOFChar(token), 0).parse(state)
                //Node::error(state.input.chomp(0)),
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
    map(node(NodeId::VIRTUAL, f), move |mut node| {
        node.alias.push(name);
        node
    })
}

pub fn node(name: NodeId, f: impl Fn(&mut State)) -> impl Parser {
    move |state: &mut State| {
        let n = Node {
            name,
            span: state.input.clone(),
            children: vec![],
            alias: vec![],
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
