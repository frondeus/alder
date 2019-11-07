#![allow(dead_code)]

use self::problem::{Ctx, DeadEnds};
use std::iter::FromIterator;

pub mod chars;
pub mod errors;
pub mod offset;
pub mod problem;

pub enum NoContext {}
pub enum NoProblem {}

pub type Result<'a, T, C = NoContext, P = NoProblem> =
    std::result::Result<(T, &'a str), DeadEnds<'a, C, P>>;

pub trait Parser<'a, C, P> {
    type T;
    fn parse(&self, i: &'a str) -> Result<'a, Self::T, C, P>;

    fn map<T2, F>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::T) -> T2,
        Self: Sized,
    {
        Map { p: self, f }
    }

    fn ctx(self, c: C) -> Ctx<Self, C>
    where
        C: Copy,
        Self: Sized,
    {
        Ctx { p: self, c }
    }
}

impl<'a, T, P, C, F> Parser<'a, C, P> for F
where
    F: Fn(&'a str) -> Result<'a, T, C, P>,
{
    type T = T;
    fn parse(&self, i: &'a str) -> Result<'a, T, C, P> {
        self(i)
    }
}

pub struct Map<P1, F> {
    p: P1,
    f: F,
}
impl<'a, P, C, P1, F, T> Parser<'a, C, P> for Map<P1, F>
where
    P1: Parser<'a, C, P>,
    F: Fn(P1::T) -> T,
{
    type T = T;

    fn parse(&self, i: &'a str) -> Result<'a, Self::T, C, P> {
        let f = &self.f;
        let (p1, i) = self.p.parse(i)?;
        let p2 = f(p1);
        let t = p2;
        Ok((t, i))
    }
}

//Recognize

pub fn recognize<'a, C, P, T>(
    p: impl Parser<'a, C, P, T = T>,
) -> impl Parser<'a, C, P, T = &'a str> {
    use crate::offset::Offset;
    move |i| {
        let (t, r) = p.parse(i)?;
        let index = i.offset(r);
        Ok((&i[..index], r))
    }
}

// Opt
pub fn opt<'a, C, P, T>(p: impl Parser<'a, C, P, T = T>) -> impl Parser<'a, C, P, T = Option<T>> {
    move |i| {
        let res = p.parse(i);
        match res {
            Ok((t, r)) => Ok((Some(t), r)),
            _ => Ok((None, i)),
        }
    }
}

//OR
pub trait Alt<'a, C, P> {
    type T;
    fn alt(&self, i: &'a str) -> Result<'a, Self::T, C, P>;
}

pub struct Or<T> {
    tuple: T,
}

pub trait IntoParser {
    fn or(self) -> Or<Self>
    where
        Self: Sized;
}

impl<'a, C, P, TP> Parser<'a, C, P> for Or<TP>
where
    TP: Alt<'a, C, P>,
{
    type T = TP::T;

    fn parse(&self, i: &'a str) -> Result<'a, Self::T, C, P> {
        self.tuple.alt(i)
    }
}

pub fn or<'a, P, C, P1, P2, T>(p1: P1, p2: P2) -> impl Parser<'a, C, P, T = T>
where
    P1: Parser<'a, C, P, T = T>,
    P2: Parser<'a, C, P, T = T>,
{
    (p1, p2).or()
}

macro_rules! impl_alt {
    () => ();
    ($($name:ident)+) => (
        impl<'a, $($name),*> IntoParser for ($($name,)*) {
            fn or(self) -> Or<Self> where Self: Sized { Or { tuple: self } }
        }

        #[allow(non_snake_case)]
        impl <'a, P, C, T, $($name),* > Alt<'a, C, P> for ($($name,)*)
        where
            $($name: Parser<'a, C, P, T = T>),*
        {
            type T = T;

            fn alt(&self, i: &'a str) -> Result<'a, Self::T, C, P> {
                let ($($name,)*) = self;
                let mut dead_ends = vec![];

                $(
                    match $name.parse(i) {
                        Ok(o) => return Ok(o),
                        Err(e) if e.failure() => return Err(e),
                        Err(mut e) => {
                            dead_ends.append(&mut e);
                        }
                    }
                )*

                Err(dead_ends.into())
            }
        }
    );
}

impl_alt! {AP BP}
impl_alt! {AP BP CP}
impl_alt! {AP BP CP DP}
impl_alt! {AP BP CP DP EP}
impl_alt! {AP BP CP DP EP FP}
impl_alt! {AP BP CP DP EP FP GP}
impl_alt! {AP BP CP DP EP FP GP HP}

//AND
macro_rules! impl_parser_tuple {
    () => ();
    ($($name:ident)+) => (
        #[allow(non_snake_case)]
        impl <'a, P, C, $($name),* > Parser<'a, C, P> for ($($name,)*)
        where
            $($name: Parser<'a, C, P>),*
        {
            type T = ($($name::T,)*);

            fn parse(&self, input: &'a str) -> Result<'a, Self::T, C, P> {
                let ($($name,)*) = self;
                let i = input;
                $(let ($name, i) = $name.parse(i)?;)*
                let o = ($($name,)*);
                Ok((o, i))
            }
        }
    );
}

impl_parser_tuple! {AP BP}
impl_parser_tuple! {AP BP CP}
impl_parser_tuple! {AP BP CP DP}
impl_parser_tuple! {AP BP CP DP EP}
impl_parser_tuple! {AP BP CP DP EP FP}
impl_parser_tuple! {AP BP CP DP EP FP GP}
impl_parser_tuple! {AP BP CP DP EP FP GP HP}

pub fn preceded<'a, C, P, T>(
    left: impl Parser<'a, C, P>,
    right: impl Parser<'a, C, P, T = T>,
) -> impl Parser<'a, C, P, T = T> {
    (left, right).map(|(_, b)| b)
}

pub fn terminated<'a, C, P, T>(
    left: impl Parser<'a, C, P, T = T>,
    right: impl Parser<'a, C, P>,
) -> impl Parser<'a, C, P, T = T> {
    (left, right).map(|(a, _)| a)
}

//Fixme: Maybe generator into iterator?
pub fn delimited<'a, C, P, T, V>(
    p1: impl Parser<'a, C, P, T = T>,
    del: impl Parser<'a, C, P>,
) -> impl Parser<'a, C, P, T = V>
where
    V: FromIterator<T>,
{
    move |mut i| {
        let mut t = vec![];
        loop {
            match p1.parse(i) {
                Ok((t1, rest)) => {
                    t.push(t1);
                    i = rest;
                }
                Err(e) if e.failure() => return Err(e),
                Err(e) => break,
            }
            match del.parse(i) {
                Ok((_, rest)) => {
                    i = rest;
                }
                Err(e) if e.failure() => return Err(e),
                Err(e) => break,
            }
        }
        let t = V::from_iter(t.into_iter());
        Ok((t, i))
    }
}
