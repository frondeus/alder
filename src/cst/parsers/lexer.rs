use crate::*;
use derive_more::Display;
// TODO:
//use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Display, Clone)]
enum LexerError {
    #[display(fmt = "I expected `{}`", _0)]
    UnexpectedToken(&'static str),
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
                raise(LexerError::UnexpectedToken(token), size).parse(state)
            }
        } else {
            raise(LexerError::UnexpectedToken(token), i_size).parse(state)
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
