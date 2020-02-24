use crate::*;
use derive_more::Display;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Display, Clone)]
enum LexerError {
    #[display(fmt = "I expected `{}`", _0)]
    UnexpectedToken(&'static str),
}

pub mod utf {
    use super::*;

    pub fn peek_nth(len: usize) -> impl Parser<Input> {
        move |state: &mut State| {
            let mut iter = state.input.graphemes_idx();

            let (offset, len) = iter.nth(len).map(|(o, g)| (o, g.len())).unwrap_or_default();

            let mut output = state.input.clone();
            output.range.0 += offset;
            output.range.1 = len;

            output
        }
    }

    pub fn nth(len: usize) -> impl Parser<Input> {
        move |state: &mut State| {
            let output = peek_nth(len).parse(state);
            state.input.range.0 += output.range.0 + output.range.1;
            state.input.range.1 -= output.range.0 + output.range.1;

            output
        }
    }

    pub fn next() -> impl Parser<Input> {
        nth(0)
    }

    pub fn peek(len: usize) -> impl Parser<Input> {
        move |state: &mut State| {
            let iter = state.input.graphemes_idx();
            let (offset, grapheme) = iter.take(len).last().unwrap_or_default();
            let mut output = state.input.clone();
            let len = offset + grapheme.len();
            output.range.1 = len;
            output
        }
    }

    pub fn chomp(len: usize) -> impl Parser<Input> {
        move |state: &mut State| {
            let output = peek(len).parse(state);
            state.input.range.0 += output.range.1;
            state.input.range.1 -= output.range.1;
            output
        }
    }

    pub fn chomp_while(f: impl Fn(&str) -> bool) -> impl Parser<Input> {
        move |state: &mut State| {
            let mut len = 0usize;
            loop {
                let current = peek_nth(len).parse(state);
                let current = match current.as_ref() {
                    "" => return chomp(len).parse(state),
                    c => c,
                };
                let valid = f(current);
                if valid {
                    len += 1;
                } else {
                    return chomp(len).parse(state);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test_case::test_case;
        const INPUT: &str = "a\u{310}e\u{301}o\u{308}\u{332}\r\n";

        #[test_case(peek_nth(0), "a\u{310}", "a\u{310}e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(peek_nth(1), "e\u{301}", "a\u{310}e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(peek_nth(3), "\r\n", "a\u{310}e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(peek_nth(4), "", "a\u{310}e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(nth(0), "a\u{310}", "e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(nth(1), "e\u{301}", "o\u{308}\u{332}\r\n")]
        #[test_case(nth(3), "\r\n", "")]
        #[test_case(nth(4), "", "a\u{310}e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(next(), "a\u{310}", "e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(peek(0), "", "a\u{310}e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(peek(1), "a\u{310}", "a\u{310}e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(
            peek(3),
            "a\u{310}e\u{301}o\u{308}\u{332}",
            "a\u{310}e\u{301}o\u{308}\u{332}\r\n"
        )]
        #[test_case(
            peek(99),
            "a\u{310}e\u{301}o\u{308}\u{332}\r\n",
            "a\u{310}e\u{301}o\u{308}\u{332}\r\n"
        )]
        #[test_case(chomp(0), "", "a\u{310}e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(chomp(1), "a\u{310}", "e\u{301}o\u{308}\u{332}\r\n")]
        #[test_case(chomp(3), "a\u{310}e\u{301}o\u{308}\u{332}", "\r\n")]
        #[test_case(chomp(99), "a\u{310}e\u{301}o\u{308}\u{332}\r\n", "")]
        #[test_case(chomp_while(|c| { c != "\n" && c != "\r\n" }),               "a\u{310}e\u{301}o\u{308}\u{332}",  "\r\n")]
        fn test_parser(p: impl Parser<Input>, expected: &'static str, expected_rest: &'static str) {
            let mut state: State = INPUT.into();
            let actual = p.parse(&mut state);
            assert_eq!(actual.as_ref(), expected);
            let rest = state.input;
            assert_eq!(rest.as_ref(), expected_rest);
        }
    }
}

impl Parser for &'static str {
    fn parse(&self, state: &mut State) -> Node {
        token(self).parse(state)
    }
}

pub fn token(token: &'static str) -> impl Parser {
    let token_len = token.graphemes(true).count();

    move |state: &mut State| {
        let output = utf::peek(token_len).parse(state);
        match output {
            n if n.as_ref() == token => {
                utf::chomp(token_len).parse(state);
                Node::token(n)
            }
            n => raise(LexerError::UnexpectedToken(token), n.len()).parse(state),
        }
    }
}

pub fn recognize(name: NodeId, parser: impl Parser<Input>) -> impl Parser {
    move |state: &mut State| {
        let output = parser.parse(state);

        match output {
            result if !result.is_empty() => Node {
                name,
                span: result,
                children: vec![],
                alias: vec![],
            },
            _ => none().parse(state),
        }
    }
}
