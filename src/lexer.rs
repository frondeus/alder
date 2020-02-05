use crate::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerProblem {
    #[error("Unexpected EOF")]
    UnexpectedEOF,

    #[error("Unexpected '{0}'")]
    Unexpected(char),
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

/*

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use crate::Rest;

    type Problem<'a> = &'a str;

    fn take_twice<'a>() -> impl Parser<'a, Problem<'a>, Output = StrOutput<'a>> {
        (
            take(2, |_| "Expected 2 chars"),
            take(3, |_| "Expected 3 chars")
        )
        .map(|(_a, b)| b)
    }

    #[test_case(take(1, |_| "Expected 1 char found EOF"), "ala ma kota" => ("a", "la ma kota"))]
    #[test_case(take(1, |_| "Expected 1 char found EOF"), "" => panics "Expected 1 char found EOF")]
    #[test_case(take(3, |_| "Expected 3 chars found only 2"), "ab" => panics "Expected 3 chars found only 2")]
    #[test_case(take_twice(), "a" => panics "Expected 2 chars at `a`")]
    fn tests<'a>(parser: impl Parser<'a, Problem<'a>, Output = StrOutput<'a>>, input: Input<'a>) -> (StrOutput<'a>, Rest<'a>) {
        let res = parser.parse(input).unwrap_display();
        dbg!(res)
    }
}
*/
