use crate::Input;
use itertools::Itertools;
use std::cmp::{Ordering, PartialOrd};
use std::fmt::{Display, Error, Formatter};
use std::str::Lines;
use termion::color::Color;
use termion::{color, style};

pub type DisplayString = String;

pub struct FancyCodeEntry {
    span: Span,
    desc: Option<DisplayString>,
    color: String,
}

pub struct FancyCode {
    src: Input,
    entries: Vec<FancyCodeEntry>,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.offset == other.offset
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    pub from: Position,
    pub to: Position,
}

impl Position {
    pub fn build(offset: usize, lines: Lines) -> Self {
        let mut off = offset;
        let mut line = 0;
        let mut column = 0;

        for (j, l) in lines.enumerate() {
            if off <= l.len() {
                line = j;
                column = off;
                break;
            } else {
                off = off - l.len() - 1;
                line = j + 1;
            }
        }

        Self {
            line,
            column,
            offset,
        }
    }
}

impl<'a> From<&'a Input> for Span {
    fn from(input: &'a Input) -> Self {
        let full = input.full();
        let full = full.as_ref();
        let from = Position::build(input.range.0, full.lines());
        let to = Position::build(input.range.0 + input.range.1, full.lines());
        Self { from, to }
    }
}

impl FancyCode {
    pub fn new(input: &Input) -> Self {
        Self {
            src: input.clone(),
            entries: vec![],
        }
    }
    pub fn with_desc(
        mut self,
        span: &Input,
        desc: impl Into<DisplayString>,
        color: impl Color + Copy + Clone,
    ) -> Self {
        let span = Span::from(span);
        let color = format!("{}", color::Fg(color));
        self.entries.push(FancyCodeEntry {
            span,
            desc: Some(desc.into()),
            color,
        });
        self
    }

    fn eol<'a>(line: usize, lines: &[&'a str]) -> &'static str {
        if lines.len() <= line + 1 {
            "\\EOF"
        } else {
            "\\n"
        }
    }
}

impl Display for FancyCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        let lines = self.src.as_ref().lines().collect::<Vec<_>>();
        if self.entries.is_empty() {
            return Ok(());
        }

        let span = self
            .entries
            .iter()
            .map(|entry| entry.span)
            .fold1(|acc, span| {
                let from = if acc.from < span.from {
                    acc.from
                } else {
                    span.from
                };
                let to = if acc.to > span.to { acc.to } else { span.to };

                Span { from, to }
            })
            .unwrap(); // is not empty

        let line_digits = span.to.line.to_string().len() + 1;
        let lines = &lines[span.from.line..=span.to.line];

        for (ln, line) in lines.iter().enumerate() {
            let ln = ln + span.from.line;
            let col = line.len();
            write!(
                f,
                "{}{: >width$} |{}{}{}",
                color::Fg(color::Cyan),
                ln,
                color::Fg(color::LightWhite),
                style::Bold,
                line,
                width = line_digits
            )?;
            writeln!(
                f,
                "{}{}{}{}",
                style::Reset,
                color::Fg(color::LightBlack),
                Self::eol(ln, lines),
                style::Reset
            )?;
            for entry in self
                .entries
                .iter()
                .filter(|entry| entry.span.from.line <= ln && ln <= entry.span.to.line)
            {
                write!(
                    f,
                    "{}{: >width$} |{}",
                    color::Fg(color::Cyan),
                    "~",
                    style::Reset,
                    width = line_digits
                )?;
                let &from = &entry.span.from;
                let &to = &entry.span.to;
                let ws_len = if ln == from.line { from.column } else { 0 };

                let u_len = if ln == to.line && ln != from.line {
                    1 + to.column - ws_len
                } else if ln == to.line {
                    std::cmp::max(1, to.column - ws_len)
                } else {
                    1 + col - ws_len
                };

                write!(f, "{:width$}", "", width = ws_len)?;
                write!(f, "{}{:^>width$}", &entry.color, "", width = u_len)?;
                if ln == from.line {
                    if let Some(desc) = &entry.desc {
                        write!(f, " {}", desc)?;
                    }
                }
                writeln!(f, "{}", style::Reset)?;
            }
        }

        Ok(())
    }
}
