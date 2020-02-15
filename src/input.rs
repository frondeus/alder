use crate::*;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub struct Input {
    src: Arc<str>,
    pub(crate) range: (usize, usize), // from, to
}

impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)?;
        write!(f, "{:?}", &self.range)?;
        Ok(())
    }
}

//#[cfg(test)] TODO
impl Input {
    pub fn new_test(src: &str, range: (usize, usize)) -> Self {
        let src = src.into();
        Self { src, range }
    }
}

impl<'a> From<&'a str> for Input {
    fn from(input: &'a str) -> Input {
        let src: Arc<str> = Arc::from(input);
        let range = (0, src.len() - 1);
        Self { src, range }
    }
}

impl Offset for Input {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.range.0;
        let snd = second.range.0;

        snd as usize - fst as usize
    }
}

impl AsRef<str> for Input {
    fn as_ref(&self) -> &str {
        &self.src[self.range.0..=self.range.1]
    }
}

impl Input {
    pub fn len(&self) -> usize {
        if self.range.1 >= self.range.0 {
            self.range.1 - self.range.0 + 1
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn peek_str(&self, len: usize) -> &str {
        let from = self.range.0;
        let to = from + len - 1;
        &self.src[from..=to]
    }

    pub fn peek(&self) -> Option<char> {
        self.as_ref().chars().next()
    }

    pub fn chomp(&mut self, len: usize) -> Self {
        if len == 0 {
            return self.clone();
        }

        let range = (self.range.0, self.range.0 + len - 1);
        self.range.0 += len;
        let src = self.src.clone();
        Self { src, range }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_create() {
        let i: Input = "(foo)".into();

        assert_eq!("(foo)", i.as_ref());
        assert_eq!("(foo)", i.peek_str(5));
        assert_eq!("(foo", i.peek_str(4));
        assert_eq!("(fo", i.peek_str(3));
        assert_eq!(5, i.len());

        assert_eq!(
            Input {
                src: "(foo)".into(),
                range: (0, 4)
            },
            i
        );
    }

    #[test]
    fn input_chomp_0() {
        let mut i: Input = "(foo)".into();
        let j = i.chomp(0);

        assert_eq!("(foo)", i.as_ref());
        assert_eq!("(foo)", i.peek_str(5));
        assert_eq!("(foo", i.peek_str(4));
        assert_eq!("(fo", i.peek_str(3));
        assert_eq!(5, i.len());

        assert_eq!(
            Input {
                src: "(foo)".into(),
                range: (0, 4)
            },
            i
        );

        assert_eq!("(foo)", j.as_ref());
        assert_eq!(5, j.len());
        assert_eq!(
            Input {
                src: "(foo)".into(),
                range: (0, 4)
            },
            j
        );
    }

    #[test]
    fn input_chomp_1() {
        let mut i: Input = "(foo)".into();
        let j = i.chomp(1);

        assert_eq!("foo)", i.as_ref());
        assert_eq!(4, i.len());
        assert_eq!("foo)", i.peek_str(4));
        assert_eq!("foo", i.peek_str(3));
        assert_eq!("fo", i.peek_str(2));

        assert_eq!(
            Input {
                src: "(foo)".into(),
                range: (1, 4)
            },
            i
        );

        assert_eq!("(", j.as_ref(), "j as_ref");
        assert_eq!(1, j.len(), "j len");
        assert_eq!("(", j.peek_str(1), "j peek 1");

        assert_eq!(
            Input {
                src: "(foo)".into(),
                range: (0, 0)
            },
            j
        );
    }

    #[test]
    fn input_chomp_2() {
        let mut i: Input = "(foo)".into();
        let j = i.chomp(2);

        assert_eq!("oo)", i.as_ref());
        assert_eq!(3, i.len());
        assert_eq!("oo)", i.peek_str(3));
        assert_eq!("oo", i.peek_str(2));
        assert_eq!("o", i.peek_str(1));

        assert_eq!(
            Input {
                src: "(foo)".into(),
                range: (2, 4)
            },
            i
        );

        assert_eq!("(f", j.as_ref(), "j as_ref");
        assert_eq!(2, j.len(), "j len");
        assert_eq!("(f", j.peek_str(2), "j peek 2");
        assert_eq!("(", j.peek_str(1), "j peek 1");

        assert_eq!(
            Input {
                src: "(foo)".into(),
                range: (0, 1)
            },
            j
        );
    }
}
