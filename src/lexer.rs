use crate::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerProblem {
    #[error("Unexpected EOF")]
    UnexpectedEOF,

    #[error("Unexpected '{0}'")]
    Unexpected(char),

    #[cfg(feature = "with_regex")]
    #[error("Failed regex")]
    FailedRegex
}

pub fn take<'a>(token_kind: NodeKind, len: usize) -> impl Parser<'a, Output = Node<'a>> {
    move |i: Input<'a>, state: &mut State<'a>| {
        let i_len = i.len();
        if i_len >= len {
            //We want to take n chars not n bytes.
            let mut end: usize = 0;
            i.chars().take(len).for_each(|x| end += x.len_utf8());

            let output = &i[0..end];
            let rest = &i[end..];

            (Node::token(token_kind, output), rest)
        } else {
            state.raise(LexerProblem::UnexpectedEOF, i);
            let rest = &i[i_len..];

            (Node::error(i), rest)
        }
    }
}

#[cfg(feature = "with_regex")]
pub fn regex<'a>(token_kind: NodeKind, regex: regex::Regex) -> impl Parser<'a, Output = Node<'a>> {
    move |i: Input<'a>, state: &mut State<'a>| {
        if let Some(range) = regex.find(i) {
            let s = range.start();
            let e = range.end();
            let output = &i[s..e];
            let rest = &i[e..];
            (Node::token(token_kind, output), rest)
        }
        else {
            state.raise(LexerProblem::FailedRegex, i);
            (Node::error(i), i)
        }
    }
}

pub fn token<'a>(token_kind: NodeKind, expected: char) -> impl Parser<'a, Output = Node<'a>> {
    let parser = take(token_kind, 1);
    move |i: Input<'a>, state: &mut State<'a>| {
        let (first, rest) = parser.parse_state(i, state);
        if !first.is_error() {
            let s = first.location;
            match s.chars().next() {
                Some(letter) if letter != expected => {
                    state.raise(LexerProblem::Unexpected(letter), i);
                    return (Node::error(s), rest);
                }
                _ => (),
            }
        }

        (first, rest)
    }
}

pub fn chomp_while<'a>(
    token_kind: NodeKind,
    f: impl Fn(char) -> bool,
) -> impl Parser<'a, Output = Node<'a>> {
    move |i: &'a str, _state: &mut State<'a>| {
        let mut len = 0;
        loop {
            let c = &i[len..].chars().next();
            match c {
                Some(letter) if f(*letter) => {
                    len += letter.len_utf8();
                }
                _ => break,
            }
        }

        let result = &i[0..len];
        let rest = &i[len..];

        (Node::token(token_kind, result), rest)
    }
}

pub fn peek_char_2<'a>(f: impl Fn(char)
    -> Box<dyn Parser<'a, Output = NodeVec<'a>> + 'a>) -> impl Parser<'a, Output = NodeVec<'a>> {

    move |i: Input<'a>, state: &mut State<'a>| {
        let mut end: usize = 0;
        let i_len = i.chars().take(1)
            .fold(0, |acc, x| {
                end += x.len_utf8();
                acc + 1
            });

        let len = 1;

        if i_len >= len {
            let peek = &i[0..end].chars().next().unwrap();
            let parser = f(*peek);
            parser.parse_state(i, state)
        }
        else {
            state.raise(LexerProblem::UnexpectedEOF, i);
            let rest = &i[end..];
            (NodeVec(vec![]), rest)
        }
    }

}

pub fn peek_char<'a>(f: impl Fn(char)
    -> Box<dyn Parser<'a, Output = Node<'a>> + 'a>) -> impl Parser<'a, Output = Node<'a>> {

    peek(1, move |i| f(i.chars().next().unwrap()))
}

pub fn peek<'a>(len: usize, f: impl Fn(Input<'a>)
    -> Box<dyn Parser<'a, Output = Node<'a>> + 'a>) -> impl Parser<'a, Output = Node<'a>> {
    move |i: Input<'a>, state: &mut State<'a>| {
        let mut end: usize = 0;
        let i_len = i.chars().take(len)
            .fold(0, |acc, x| {
                end += x.len_utf8();
                acc + 1
            });

        if i_len >= len {
            let peek = &i[0..end];
            let parser = f(peek);
            parser.parse_state(i, state)
        }
        else {
            state.raise(LexerProblem::UnexpectedEOF, i);
            let rest = &i[end..];
            (Node::error(i), rest)
        }
    }
}
