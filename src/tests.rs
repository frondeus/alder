use super::*;
use crate::chars::{char, take, CharProblem};
use std::fmt::Debug;
use test_case::test_case;

#[derive(Debug)]
struct Foo<'a>(&'a str, &'a str);

#[derive(Debug, Clone, Copy)]
enum BarContext {
    Bar,
}

impl Context for BarContext {}

impl From<NoContext> for BarContext {
    fn from(_: NoContext) -> Self {
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
enum BazContext {
    Bar(BarContext),
    Baz,
}

impl Context for BazContext {}

impl From<NoContext> for BazContext {
    fn from(_: NoContext) -> Self {
        unreachable!()
    }
}

impl From<BarContext> for BazContext {
    fn from(f: BarContext) -> Self {
        BazContext::Bar(f)
    }
}

fn foo<'a>() -> impl Fn(&'a str) -> Result<Foo<'a>, CharProblem<'a>> {
    map(and(take(1), take(2)), |(t1, t2)| Foo(t1, t2))
}

fn bar<'a>() -> impl Fn(&'a str) -> Result<Foo<'a>, CharProblem<'a>, BarContext> {
    context(BarContext::Bar, foo())
}

fn baz<'a>() -> impl Fn(&'a str) -> Result<(Foo<'a>, &'a str), CharProblem<'a>, BazContext> {
    context(BazContext::Baz, and(bar(), take(1)))
}

#[test_case(take(1), "abc" => matches ("a", _) )]
#[test_case(foo(),   "abc" => matches ( Foo("a", "bc"), _) )]
#[test_case(bar(),   "abc" => matches ( Foo("a", "bc"), _) )]
#[test_case(bar(),   "ab"  => panics "[Bar]" )]
#[test_case(baz(),   ""    => panics "[Baz, Bar(Bar)]" )]
fn test<'a, T: Debug, Problem: Debug, Context: Debug>(
    p: impl Fn(&'a str) -> Result<'a, T, Problem, Context>,
    input: &'a str,
) -> (T, &'a str) {
    p(input).unwrap()
}
