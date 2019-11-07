use crate::offset::Offset;
use crate::{Parser, Result};
use colored::{ColoredString, Colorize};
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};
use std::iter::repeat;
use std::ops::{Deref, DerefMut};

pub struct Ctx<P1, C> {
    pub(crate) c: C,
    pub(crate) p: P1,
}
impl<'a, P, C, P1> Parser<'a, C, P> for Ctx<P1, C>
where
    P1: Parser<'a, C, P>,
    C: Copy,
{
    type T = P1::T;

    fn parse(&self, i: &'a str) -> Result<'a, Self::T, C, P> {
        self.p.parse(i).map_err(|mut e| {
            e.iter_mut().for_each(|e| e.add_context(self.c, i));
            e
        })
    }
}

pub fn context<'a, P1, P, C, T>(c: C, p: P1) -> impl Parser<'a, C, P, T = T>
where
    P1: Parser<'a, C, P, T = T>,
    C: Copy,
{
    Ctx { c, p }
}

#[derive(Debug)]
pub struct DeadEnds<'a, C, P> {
    pub ends: Vec<DeadEnd<'a, C, P>>,
}

impl<'a, C, P> DeadEnds<'a, C, P> {
    pub fn failure(&self) -> bool {
        self.ends.iter().any(|e| e.failure)
    }
}

impl<'a, P, C> Deref for DeadEnds<'a, C, P> {
    type Target = Vec<DeadEnd<'a, C, P>>;

    fn deref(&self) -> &Self::Target {
        &self.ends
    }
}

impl<'a, P, C> DerefMut for DeadEnds<'a, C, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ends
    }
}

impl<'a, P, C> From<Vec<DeadEnd<'a, C, P>>> for DeadEnds<'a, C, P> {
    fn from(ends: Vec<DeadEnd<'a, C, P>>) -> Self {
        Self { ends }
    }
}

#[derive(Debug)]
pub struct DeadEnd<'a, C, P> {
    pub problem: P,
    pub failure: bool,
    pub input: &'a str,
    pub context: Vec<(&'a str, C)>,
}

impl<'a, P, C> DeadEnd<'a, C, P> {
    pub fn new(problem: P, input: &'a str) -> Self {
        Self {
            problem,
            input,
            failure: false,
            context: vec![],
        }
    }

    pub fn vec(problem: P, input: &'a str) -> DeadEnds<'a, C, P> {
        DeadEnds {
            ends: vec![Self::new(problem, input)],
        }
    }

    pub fn add_context(&mut self, c: C, input: &'a str) {
        //if !self.failure {
        //self.context.insert(0, (input, c));
        self.context.push((input, c));
        //}
    }

    pub fn cut(&mut self) {
        self.failure = true;
    }
}

pub fn cut<'a, C, P, T>(p: impl Parser<'a, C, P, T = T>) -> impl Parser<'a, C, P, T = T> {
    move |i| {
        p.parse(i).map_err(|mut d| {
            d.iter_mut().for_each(|i| i.cut());
            d
        })
    }
}

pub struct DisplayInputEntry<'a> {
    whre: &'a str,
    desc: Option<ColoredString>,
}

pub struct DisplayInput<'a> {
    input: &'a str,
    entries: Vec<DisplayInputEntry<'a>>,
}

impl<'a> DisplayInput<'a> {
    pub fn new(input: &'a str, whre: &'a str) -> Self {
        Self {
            input,
            entries: vec![DisplayInputEntry { whre, desc: None }],
        }
    }
    pub fn with_desc(mut self, desc: impl Into<ColoredString>) -> Self {
        self.entries[0].desc = Some(desc.into()); //TODO
        self
    }

    fn eol(line: usize, lines: &[&'a str]) -> &'static str {
        if lines.len() <= line + 1 {
            "\\EOF"
        } else {
            "\\n"
        }
    }

    /// Output: (line, column)
    fn position(&self, lines: &[&'a str], whre: &'a str) -> (usize, usize) {
        let mut offset = self.input.offset(whre);
        let mut line = 0;
        let mut column = 0;

        for (j, l) in lines.iter().enumerate() {
            if offset <= l.len() {
                line = j;
                column = offset;
                break;
            } else {
                offset = offset - l.len() - 1;
                line = j + 1;
            }
        }

        (line, column)
    }
}

impl<'a> Display for DisplayInput<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        let input = self.input;
        let lines = input.lines().collect::<Vec<_>>();

        for entry in self.entries.iter() {
            let (line, column) = self.position(&lines, entry.whre);

            if line > 0 {
                if let Some(prev) = lines.get(line - 1) {
                    write!(f, "{}", format!("{}  | ", line - 1).cyan())?;
                    write!(f, "{}", (prev).bright_white())?;
                    writeln!(f, "{}", "\\n".bright_black())?;
                }
            }

            write!(f, "{}", format!("{}  | ", line).cyan())?;
            if let Some(this) = lines.get(line) {
                write!(f, "{}", this.bright_white())?;
            }
            writeln!(f, "{}", Self::eol(line, &lines).bright_black())?;
            write!(f, "{}", "   | ".cyan())?;
            if column > 0 {
                write!(f, "{}", &repeat(' ').take(column).collect::<String>())?;
            }
            let desc = entry.desc.clone().unwrap_or_else(|| "^".normal());
            writeln!(f, "{}", &desc.bold())?;
            if let Some(next) = lines.get(line + 1) {
                write!(f, "{}", format!("{}  | ", line + 1).cyan())?;
                write!(f, "{}", (next).bright_white())?;
                writeln!(f, "{}", Self::eol(line + 1, &lines).bright_black())?;
            }
        }

        Ok(())
    }
}
