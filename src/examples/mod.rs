use super::*;
use crate::chars::{spaces, token};
use crate::errors::ResultExt;
use crate::examples::report::Report;
use crate::problem::cut;
use colored::Colorize;
use derive_more::Display;
use std::fmt::Debug;
use test_case::test_case;

#[derive(Debug)]
enum Cons {
    Cons,
    Nil,
    Pair(Box<Cons>, Box<Cons>),
}

#[derive(Debug, Display, Clone, PartialEq)]
pub enum Problem {
    #[display(fmt = "I expected 'nil'")]
    ExpectedNil,

    #[display(fmt = "I expected '('")]
    ExpectedOpenBracket,

    #[display(fmt = "I expected '.'")]
    ExpectedDot,

    #[display(fmt = "I expected ')'")]
    ExpectedCloseBracket,
}

impl Problem {
    pub fn name(&self) -> &'static str {
        use Problem::*;
        match self {
            ExpectedNil => "()",
            ExpectedOpenBracket => "(",
            ExpectedCloseBracket => ")",
            ExpectedDot => ".",
        }
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum ConsContext {
    #[display(fmt = "I tried to parse nil value")]
    Nil,
    #[display(
        fmt = "I tried to parse cons pair {}",
        r#""(<expr> . <expr>)".white().bold()"#
    )]
    Pair,
    #[display(fmt = "I tried to parse expression")]
    Expression,
}

mod report;

fn nil<'a>() -> impl Parser<'a, ConsContext, Problem, T = Cons> {
    //tag("()", Problem::ExpectedNil)
    (
        token('(', Problem::ExpectedOpenBracket),
        token(')', Problem::ExpectedCloseBracket),
    )
        .ctx(ConsContext::Nil)
        .map(|_| Cons::Nil)
}

fn cons<'a>() -> impl Parser<'a, ConsContext, Problem, T = Cons> {
    |i| preceded(spaces(), or(nil(), pair()).ctx(ConsContext::Expression)).parse(i)
}

fn pair<'a>() -> impl Parser<'a, ConsContext, Problem, T = Cons> {
    (
        token('(', Problem::ExpectedOpenBracket),
        cons(),
        spaces(),
        token('.', Problem::ExpectedDot),
        cut((cons(), spaces(), token(')', Problem::ExpectedCloseBracket))),
    )
        .map(|(_, lhs, _, _, (rhs, _, _))| Cons::Pair(Box::new(lhs), Box::new(rhs)))
        .ctx(ConsContext::Pair)
}

#[test_case(cons(),   "("    ; "eof" )]
#[test_case(cons(),   r#"(
    ()
    .
    (a
)"#   ; "pair error" )]
#[test_case(cons(),   r#"(
    ()
    .
    ()
"#  ; "missing close paren" )]
fn test_errors<'a, T: Debug>(p: impl Parser<'a, ConsContext, Problem, T = T>, input: &'a str) {
    let err = p
        .parse(input)
        .map_err(|ends| Report::new(input, ends))
        .unwrap_display_err();

    insta::assert_display_snapshot!(err);
}

#[test_case(cons(),   "()"   => matches ( Cons::Nil, _) )]
fn test<'a, T: Debug>(
    p: impl Parser<'a, ConsContext, Problem, T = T>,
    input: &'a str,
) -> (T, &'a str) {
    p.parse(input)
        .map_err(|ends| Report::new(input, ends))
        .unwrap_display()
}
