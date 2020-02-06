use crate::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerProblem {
    #[error("Unexpected EOF")]
    UnexpectedEOF,

    #[error("I expected '{0}'")]
    ExpectedTag(&'static str),

    #[error("I expected '{0}'")]
    ExpectedToken(char),

    #[cfg(feature = "with_regex")]
    #[error("Failed regex")]
    FailedRegex,
}

pub fn tag<'a>(kind: NodeKind, tag: &'static str) -> impl Parser<'a> {
    move |mut state: State<'a>| {
        let size = tag.len();
        let i_size = state.input.len();
        if i_size >= size {
            let t = &state.input[..size];
            if t == tag {
                let node = Node::token(kind, state.chomp(size));
                state.add(node);
                return state;
            } else {
                return state.raise(LexerProblem::ExpectedTag(tag), t);
            }
        }
        let input = state.input;
        state.raise(LexerProblem::ExpectedTag(tag), input)
    }
}

impl<'a> Parser<'a> for char
{
    fn parse_state(&self, state: State<'a>) -> State<'a> {
        token(*self).parse_state(state)
    }
}

pub fn token<'a>(token: char) -> impl Parser<'a> {
    move |mut state: State<'a>| {
        let next = state.input.chars().next();
        match next {
            Some(n) if n == token => {
                let node = Node::token(NodeKind::TOKEN, state.chomp(n.len_utf8()));
                state.add(node);
                state
            }
            Some(n) => {
                let loc = state.chomp(n.len_utf8());
                state.raise(LexerProblem::ExpectedToken(token), loc)
            }
            None => {
                let loc = state.chomp(0);
                state.raise(LexerProblem::ExpectedToken(token), loc)
            }
        }
    }
}

pub fn chomp_while<'a>(token_kind: NodeKind, f: impl Fn(char) -> bool) -> impl Parser<'a> {
    move |mut state: State<'a>| {
        let mut len = 0;
        loop {
            let c = &state.input[len..].chars().next();
            match c {
                Some(letter) if f(*letter) => {
                    len += letter.len_utf8();
                }
                _ => break,
            }
        }

        if len != 0 {

        let result = state.chomp(len);

        let node = Node::token(token_kind, result);
        state.add(node);
        }
        state
    }
}

#[cfg(feature = "with_regex")]
pub fn regex<'a>(token_kind: NodeKind, regex: regex::Regex) -> impl Parser<'a> {
    move |mut state: State<'a>| {
        if let Some(range) = regex.find(state.input) {
            let e = range.end();
            let output = state.chomp(e);
            let node = Node::token(token_kind, output);
            state.add(node);
            state
        }
        else {
            let loc = state.chomp(0);
            state.raise(LexerProblem::FailedRegex, loc)
        }
    }
}
