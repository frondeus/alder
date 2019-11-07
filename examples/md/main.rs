#![feature(drain_filter)]
use crate::report::Report;
use alder::chars::{char_where, chomp_while, spaces, token};
use alder::errors::ResultExt;
use alder::problem::cut;
use alder::{delimited, opt, or, preceded, recognize, terminated, IntoParser, Parser};
use colored::Colorize;
use derive_more::Display;
use std::collections::BTreeMap;
use std::fmt::Debug;
use test_case::test_case;

mod report;

#[derive(Debug)]
enum Value<'a> {
    ConstNum(f64),
    ConstStr(&'a str),
    ConstArr(Vec<Value<'a>>),
    ConstObj(BTreeMap<&'a str, Value<'a>>),
}

#[derive(Debug, Display, Clone, PartialEq)]
pub enum Problem {
    #[display(fmt = "I expected number like 0, 1, -2, 0.5")]
    ExpectedLiteralNumber,

    #[display(fmt = "I expected '-'")]
    ExpectedMinus,

    #[display(fmt = "I expected '.'")]
    ExpectedDot,

    #[display(fmt = r#"I expected '"'"#)]
    ExpectedDoublequote,

    #[display(fmt = r#"I expected '['"#)]
    ExpectedOpenBracket,

    #[display(fmt = r#"I expected ']'"#)]
    ExpectedCloseBracket,

    #[display(fmt = r#"I expected ','"#)]
    ExpectedComma,

    #[display(fmt = "I expected '{{'")]
    ExpectedCurlyOpenBracket,

    #[display(fmt = "I expected '}}'")]
    ExpectedCurlyCloseBracket,

    #[display(fmt = "I expected '='")]
    ExpectedEqual,

    #[display(fmt = "I expected identifier")]
    ExpectedIdentifier,
}

impl Problem {
    pub fn name(&self) -> &'static str {
        use Problem::*;
        match self {
            ExpectedLiteralNumber => "num",
            ExpectedIdentifier => "ident",
            ExpectedMinus => "-",
            ExpectedDot => ".",
            ExpectedDoublequote => r#"""#,
            ExpectedOpenBracket => "[",
            ExpectedCloseBracket => "]",
            ExpectedComma => ",",
            ExpectedCurlyOpenBracket => "{",
            ExpectedCurlyCloseBracket => "}",
            ExpectedEqual => "=",
        }
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum Context {
    //#[display(fmt = "I tried to parse number {}", r#""<name> = <value>".white().bold()"#)]
    #[display(fmt = "I tried to parse const value")]
    ConstValue,

    #[display(fmt = "I tried to parse const number")]
    ConstNumber,

    #[display(fmt = "I tried to parse const string")]
    ConstStr,

    #[display(fmt = "I tried to parse const array")]
    ConstArray,

    #[display(fmt = "I tried to parse const obj")]
    ConstObj,
}

fn c_value<'a>() -> impl Parser<'a, Context, Problem, T = Value<'a>> {
    |i| {
        preceded(
            spaces(),
            (c_number(10), c_string(), c_array(), c_obj()).or(),
        )
        .ctx(Context::ConstValue)
        .parse(i)
    }
}

fn c_string<'a>() -> impl Parser<'a, Context, Problem, T = Value<'a>> {
    preceded(
        token('"', Problem::ExpectedDoublequote),
        cut(terminated(
            chomp_while(|c| c != '"'),
            token('"', Problem::ExpectedDoublequote),
        )),
    )
    .map(Value::ConstStr)
    .ctx(Context::ConstStr)
}

fn c_obj<'a>() -> impl Parser<'a, Context, Problem, T = Value<'a>> {
    preceded(
        token('{', Problem::ExpectedCurlyOpenBracket),
        cut(terminated(
            delimited(c_obj_pair(), token(',', Problem::ExpectedComma)),
            token('}', Problem::ExpectedCurlyCloseBracket),
        )
        .map(Value::ConstObj)),
    )
    .ctx(Context::ConstObj)
}

fn c_obj_pair<'a>() -> impl Parser<'a, Context, Problem, T = (&'a str, Value<'a>)> {
    (
        recognize((
            char_where(
                |c| c != '=' && !c.is_whitespace(),
                Problem::ExpectedIdentifier,
            ),
            chomp_while(|c| c != '=' && !c.is_whitespace()),
        )),
        spaces(),
        token('=', Problem::ExpectedEqual),
        c_value(),
    )
        .map(|(a, _, _, b)| (a, b))
}

fn c_array<'a>() -> impl Parser<'a, Context, Problem, T = Value<'a>> {
    preceded(
        token('[', Problem::ExpectedOpenBracket),
        cut(terminated(
            delimited(c_value(), token(',', Problem::ExpectedComma)),
            token(']', Problem::ExpectedCloseBracket),
        )
        .map(Value::ConstArr)),
    )
    .ctx(Context::ConstArray)
}

fn c_number<'a>(radix: u32) -> impl Parser<'a, Context, Problem, T = Value<'a>> {
    recognize((
        opt(token('-', Problem::ExpectedMinus)),
        char_where(move |c| c.is_digit(radix), Problem::ExpectedLiteralNumber),
        chomp_while(move |c| c.is_digit(radix)),
        opt((
            token('.', Problem::ExpectedDot),
            chomp_while(move |c| c.is_digit(radix)),
        )),
    ))
    .map(|number| {
        let n = number.parse::<f64>().unwrap_or_default();
        Value::ConstNum(n)
    })
    .ctx(Context::ConstNumber)
}

#[test_case(c_value(),   "--5"    ; "minus minus digit" )]
#[test_case(c_value(),   "a"    ; "non" )]
#[test_case(c_value(),   r#""a"#    ; "string missing end" )]
#[test_case(c_value(),   r#"[5, 6"#    ; "array missing end" )]
#[test_case(c_value(),   r#"[5, 6,"#    ; "array missing end trailing comma" )]
#[test_case(c_value(),   r#"{"#    ; "obj missing end" )]
#[test_case(c_value(),   r#"{foo = 5"#    ; "obj missing end with pair" )]
#[test_case(c_value(),   r#"{foo = 5,"#    ; "obj missing end with pair trailing comma" )]
fn test_errors<'a, T: Debug>(p: impl Parser<'a, Context, Problem, T = T>, input: &'a str) {
    let res = p.parse(input).map_err(|ends| Report::new(input, ends));

    dbg!(&res);
    let err = res.unwrap_display_err();

    insta::assert_display_snapshot!(err);
}

#[test_case(c_value(),   "-5" => matches (Value::ConstNum(-5.0), _)        ; "minus digit" )]
#[test_case(c_value(),   "5"  => matches (Value::ConstNum(5.0), _)         ; "digit" )]
#[test_case(c_value(),   "5.4" => matches (Value::ConstNum(5.4), _)      ; "double" )]
#[test_case(c_value(),   r#""foo""# => matches (Value::ConstStr("foo"), _)      ; "string" )]
#[test_case(c_value(),   r#"[5, 6]"# => matches (Value::ConstArr(_), _)    ; "array" )]
#[test_case(c_value(),   r#"[5, 6,]"# => matches (Value::ConstArr(_), _)    ; "array trailing comma" )]
#[test_case(c_value(),   r#"[]"# => matches (Value::ConstArr(_), _)    ; "empty array" )]
#[test_case(c_value(),   r#"{}"# => matches (Value::ConstObj(_), _)    ; "empty obj" )]
#[test_case(c_value(),   r#"{foo = 5}"# => matches (Value::ConstObj(_), _)    ; "obj" )]
#[test_case(c_value(),   r#"{foo = 5,}"# => matches (Value::ConstObj(_), _)    ; "obj trailing comma" )]
fn test<'a, T: Debug>(p: impl Parser<'a, Context, Problem, T = T>, input: &'a str) -> (T, &'a str) {
    p.parse(input)
        .map_err(|ends| Report::new(input, ends))
        .unwrap_display()
}

fn main() {}
