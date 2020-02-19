use crate::{Input, Node, Parsed, ParseError, NodeId, Offset};
use std::fmt::{Display, Error, Formatter};

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
            for ParseError{problem, span, context} in self.errors.iter() {
                writeln!(f, "{:-^80}", " SYNTAX ERROR ")?;
                if let Some(context) = context.last() {
                    write!(f, "I was parsing {} when ", context.node)?;
                }
                writeln!(f, "found issue:")?;

                DisplayInput::new(&self.input, span.clone())
                    .with_desc(format!("{}", problem).as_str())
                    .fmt(f)?;
            }
        }
        Ok(())
    }
}

use colored::{ColoredString, Colorize};

type DisplayString = ColoredString;

pub struct DisplayInputEntry {
    span: Input,
    desc: Option<DisplayString>,
}

pub struct DisplayInput {
    input: Input,
    entries: Vec<DisplayInputEntry>,
}

// TODO: Fix value_10 and value_12. Which is - underscore is longer than line.
impl DisplayInput {
    pub fn new(input: &Input, span: Input) -> Self {
        Self {
            input: input.clone(),
            entries: vec![DisplayInputEntry { span, desc: None }],
        }
    }
    pub fn with_desc(mut self, desc: impl Into<DisplayString>) -> Self {
        self.entries[0].desc = Some(desc.into()); //TODO: I don't even remember what. Maybe more than one desc?
        self
    }

    fn eol<'a>(line: usize, lines: &[&'a str]) -> &'static str {
        if lines.len() <= line + 1 {
            "\\EOF"
        } else {
            "\\n"
        }
    }

    /// Output: (line, column)
    fn position<'a>(&self, lines: &[&'a str], span: &Input) -> (usize, usize) {
        let mut offset = self.input.offset(span);
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

impl Display for DisplayInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        let input = &self.input;
        let lines = input.as_ref().lines().collect::<Vec<_>>();
        dbg!(&lines);

        for entry in self.entries.iter() {
            let (line, column) = self.position(&lines, &entry.span);
            let len = std::cmp::max(1, entry.span.len());

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
                write!(f, "{}", &std::iter::repeat(' ').take(column).collect::<String>())?;
            }

            let underscore = std::iter::repeat("^").take(len).collect::<String>().normal();

            let desc = entry.desc.clone().unwrap_or_default();
            writeln!(f, "{} {}", underscore, &desc.bold())?;
            if let Some(next) = lines.get(line + 1) {
                write!(f, "{}", format!("{}  | ", line + 1).cyan())?;
                write!(f, "{}", (next).bright_white())?;
                writeln!(f, "{}", Self::eol(line + 1, &lines).bright_black())?;
            }
        }

        Ok(())
    }
}
