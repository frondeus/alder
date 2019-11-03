#![allow(dead_code)]

use self::problem::{Ctx, DeadEnds};

pub mod offset;
pub mod problem;
pub mod errors;
pub mod chars;

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

//OR

pub fn or<'a, P, C, P1, P2, T>(p1: P1, p2: P2) -> impl Parser<'a, C, P, T = T>
where
    P1: Parser<'a, C, P, T = T>,
    P2: Parser<'a, C, P, T = T>,
{
    move |i| {
        let mut dead_ends = vec![];
        match p1.parse(i) {
            Ok(o) => return Ok(o),
            Err(e) if e.iter().any(|d| d.failure) => {
                return Err(e);
            }
            Err(mut e) => {
                dead_ends.append(&mut e);
                match p2.parse(i) {
                    Ok(o) => return Ok(o),
                    Err(e) if e.iter().any(|d| d.failure) => {
                        return Err(e);
                    }
                    Err(mut e) => {
                        dead_ends.append(&mut e);
                    }
                }
            }
        }

        Err(dead_ends.into())
    }
}

/*
#[allow(non_snake_case)]
impl <'a, P, C, P1> Parser<'a, C, P> for &[P1]
where
P1: Parser<'a, C, P>
{
    type T = P1::T;

    fn parse(&self, i: &'a str) -> Result<'a, Self::T, C, P> {
        let mut dead_ends = vec![];
        for p in self.iter() {
            match p.parse(i) {
                Ok((o1, r1)) => return Ok((o1, r1)),
                Err(mut e) => dead_ends.append(&mut e)
            }
        }

        Err( dead_ends.into() )
    }
}
*/

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

