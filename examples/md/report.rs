use crate::{Context, Problem};
use alder::offset::Offset;
use alder::problem::{DeadEnds, DisplayInput};
use colored::Colorize;
use std::error::Error as StdError;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct Report<'a> {
    ends: DeadEnds<'a, Context, Problem>,
    root: Option<(&'a str, Context)>,
    input: &'a str,
}

impl<'a> Report<'a> {
    pub fn new(input: &'a str, ends: DeadEnds<'a, Context, Problem>) -> Self {
        Self {
            input,
            ends,
            root: None,
        }
        .filter_ends()
    }

    fn filter_ends(mut self) -> Self {
        let input = self.input;
        if self.ends.len() > 1 {
            self.root = self.ends[0].context.pop();
            self.ends.iter_mut().skip(1).map(|d| d.context.pop());
            self.ends.iter_mut().for_each(|d| {
                d.context.dedup_by(|(a_w, a_c), (b_w, b_c)| {
                    input.offset(a_w) == input.offset(b_w)
                    //match c {
                    //Context::Expression => true,
                    //_ => false
                    //}
                });
                d.context.reverse();
            });
            dbg!(&self);
        }
        self
    }
}

impl<'a> StdError for Report<'a> {}

impl<'a> Display for Report<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        dbg!(&self);
        writeln!(f, "{}", "-- SYNTAX ERROR --".red().bold())?;
        if self.ends.len() == 1 {
            let end = &self.ends[0];
            if let Some(ctx) = end.context.first() {
                writeln!(f, "{}", ctx.1)?;
                DisplayInput::new(self.input, ctx.0)
                    .with_desc("^ started here".cyan())
                    .fmt(f)?;
            }
            let desc = format!(
                "{} {}",
                "^ unexpected token.".red(),
                end.problem.to_string()
            );
            DisplayInput::new(self.input, end.input)
                .with_desc(desc.as_str())
                .fmt(f)?;
            writeln!(f, "")?;
        } else if let Some(root) = self.root {
            writeln!(f, "{}", root.1)?;
            DisplayInput::new(self.input, root.0)
                .with_desc("^ started here".cyan())
                .fmt(f)?;

            writeln!(f, "")?;
            let mut problems = self
                .ends
                .iter()
                .map(|end| end.problem.name())
                .collect::<Vec<_>>();
            problems.dedup();
            problems.sort();
            if problems.len() > 1 {
                write!(f, "I expected one of: ")?;
                write!(f, "{}", problems[0].cyan().bold())?;
                for problem in problems.iter().skip(1) {
                    write!(f, ", {}", problem.cyan().bold())?;
                }
                writeln!(f, "\n")?;
            }
            writeln!(f, "I found few possible branches:\n")?;

            for end in self.ends.iter() {
                let contexts = end.context.iter().collect::<Vec<_>>();
                write!(f, "{} ", "*".white().bold())?;

                if let Some((whre, ctx)) = contexts.first() {
                    if whre != &end.input || contexts.len() == 1 {
                        writeln!(f, "{}", ctx)?;
                    }
                    if whre != &end.input {
                        DisplayInput::new(self.input, whre)
                            .with_desc("^ started here".cyan())
                            .fmt(f)?;
                        writeln!(f, "")?;
                    }
                }

                for (i, (whre, ctx)) in contexts.iter().skip(1).enumerate() {
                    if whre != &end.input || contexts.len() == i + 2 {
                        writeln!(f, "Then {}", ctx)?;
                    }
                    if whre != &end.input {
                        DisplayInput::new(self.input, whre)
                            .with_desc("^ started here".cyan())
                            .fmt(f)?;
                        writeln!(f, "")?;
                    }
                }

                let desc = format!(
                    "{} {}",
                    "^ unexpected token.".red(),
                    end.problem.to_string()
                );
                DisplayInput::new(self.input, end.input)
                    .with_desc(desc.as_str())
                    .fmt(f)?;
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}
