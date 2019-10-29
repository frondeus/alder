use crate::problem::DeadEnd;
use crate::Parser;

pub fn take<'a, C, P: Clone>(len: usize, problem: P) -> impl Parser<'a, C, P, T = &'a str> {
    move |i: &'a str| {
        let i_len = i.len();
        if i_len < len {
            return Err(DeadEnd::vec(problem.clone(), i));
        }
        Ok((&i[0..len], &i[len..]))
    }
}

pub fn char_where<'a, C, P: Clone>(
    f: impl Fn(char) -> bool,
    problem: P,
) -> impl Parser<'a, C, P, T = char> {
    let parser = take(1, problem.clone()).map(|s| s.chars().next());
    move |i| {
        let (first, rest) = parser.parse(i)?;
        match first {
            Some(letter) if f(letter) => Ok((letter, rest)),
            _ => Err(DeadEnd::vec(problem.clone(), i)),
        }
    }
}

pub fn chomp_while<'a, C, P>(f: impl Fn(char) -> bool) -> impl Parser<'a, C, P, T = &'a str> {
    move |i: &'a str| {
        let mut len = 0;
        loop {
            let c = &i[len..].chars().next();
            match c {
                Some(letter) if f(*letter) => {
                    len += 1;
                }
                _ => break,
            }
        }

        let result = &i[0..len];
        let rest = &i[len..];
        Ok((result, rest))
    }
}

pub fn token<'a, C, P: Clone>(expected: char, problem: P) -> impl Parser<'a, C, P, T = char> {
    let parser = take(1, problem.clone()).map(|s| s.chars().next());
    move |i| {
        let (first, rest) = parser.parse(i)?;
        match first {
            Some(letter) if letter == expected => Ok((letter, rest)),
            _ => Err(DeadEnd::vec(problem.clone(), i)),
        }
    }
}

pub fn tag<C, P: Clone>(expected: &str, problem: P) -> impl Parser<C, P, T = &str> {
    move |i| {
        let len = expected.len();
        let (o, r) = take(len, problem.clone()).parse(i)?;
        if o == expected {
            return Ok((o, r));
        }
        Err(DeadEnd::vec(problem.clone(), i))
    }
}

pub fn spaces<'a, C, P>() -> impl Parser<'a, C, P, T = &'a str> {
    chomp_while(|f| f.is_whitespace())
}
/*
impl<'a, C> Parser<'a, C, CharProblem<'a>> for char {
    type T = char;

    fn parse(&self, i: &'a str) -> Result<'a, Self::T, C, CharProblem<'a>> {
        token(*self).parse(i)
    }
}


impl<'a, C> Parser<'a, C, CharProblem<'a>> for &'a str {
    type T = &'a str;

    fn parse(&self, i: &'a str) -> Result<'a, Self::T, C, CharProblem<'a>> {
        tag(self).parse(i)
    }
}
*/
