use crate::*;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub kind: NodeKind,
    pub location: Location<'a>,
    pub children: Vec<Node<'a>>,
    pub(crate) expect_children: bool
}

impl<'a> Node<'a> {
    pub fn error(location: Location<'a>) -> Self {
        Self {
            kind: NodeKind::ERROR,
            location,
            children: vec![],
            expect_children: false,
        }
    }

    pub fn token(kind: NodeKind, location: Location<'a>) -> Self {
        Self {
            kind,
            location,
            children: vec![],
            expect_children: false
        }
    }

    pub fn is_error(&self) -> bool {
        self.kind.is_error()
    }

    pub fn expect_children(&self) -> bool {
        self.expect_children
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let width = f.width().unwrap_or_default();
        if width > 0 {
            write!(f, "{:width$}", " ", width = width)?;
        }
        #[cfg(not(feature = "debug"))]
        write!(f, "{}", self.kind.0)?;
        #[cfg(feature = "debug")]
        write!(f, "{}", self.kind.0.to_uppercase())?;

        write!(f, ": ")?;
        writeln!(f, "{:?}", self.location)?;
        let c_width = width + 4;
        for child in self.children.iter() {
            write!(f, "{:width$}", child, width = c_width)?;
        }
        Ok(())
    }
}
