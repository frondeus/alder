use crate::{Input, Node, NodeId, ParseError, Parsed};
use std::fmt::{Display, Error, Formatter};
use termion::{color, style};

mod fancy_code;
use fancy_code::FancyCode;

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.as_ref())?;
        Ok(())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let width = f.width().unwrap_or_default();
        if width > 0 {
            write!(f, "{:width$}", " ", width = width)?;
        }
        write!(f, "{}", self.name.0.to_uppercase())?;

        if !self.alias.is_empty() {
            write!(f, " (")?;
            write!(
                f,
                "{}",
                self.alias
                    .iter()
                    .map(|alias| alias.0.to_uppercase())
                    .collect::<Vec<_>>()
                    .join(" ")
            )?;
            write!(f, ")")?;
        }

        write!(f, ": ")?;
        writeln!(f, "{:?}", self.span)?;
        let c_width = width + 4;
        for child in self.children.iter() {
            write!(f, "{:width$}", child, width = c_width)?;
        }
        Ok(())
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl Display for Parsed {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "```")?;
        writeln!(f, "{}", self.input)?;
        writeln!(f, "```")?;
        writeln!(f)?;
        for root in self.nodes.iter() {
            write!(f, "{}", root)?;
        }
        writeln!(f)?;
        if !self.rest.is_empty() {
            writeln!(f, "REST: {:?}", self.rest)?;
        }
        if self.errors.is_empty() {
            writeln!(f, "NO PROBLEMS")?;
        } else {
            writeln!(f, "PROBLEMS:")?;
            for ParseError {
                problem,
                span,
                context,
            } in self.errors.iter()
            {
                writeln!(
                    f,
                    "{}{:-^80}{}",
                    color::Fg(color::Red),
                    " SYNTAX ERROR ",
                    style::Reset
                )?;
                if let Some(context) = context.last() {
                    write!(f, "I was parsing {} when ", context.node)?;
                }
                writeln!(f, "found issue:")?;

                FancyCode::new(&self.input)
                    .with_desc(span, format!("{}", problem).as_str(), color::LightRed)
                    .fmt(f)?;
            }
        }
        Ok(())
    }
}
