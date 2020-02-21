use crate::*;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub struct Input {
    src: Arc<str>,
    pub(crate) range: (usize, usize), // pos, len
}

impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)?;
        write!(f, "{:?}", &self.range)?;
        Ok(())
    }
}

impl<'a> From<&'a str> for Input {
    fn from(input: &'a str) -> Input {
        let src: Arc<str> = Arc::from(input);
        let range = (0, src.len());
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
        &self.src[self.range.0..self.range.0 + self.range.1]
    }
}

impl Input {
    pub fn full(&self) -> Input {
        let src = self.src.clone();
        let len = src.len();
        let range = (0, len);
        Self { src, range }
    }

    pub fn len(&self) -> usize {
        self.range.1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn peek_str(&self, len: usize) -> &str {
        let len = std::cmp::min(len, self.len());
        let from = self.range.0;
        let to = from + len;
        &self.src[from..to]
    }

    pub fn peek(&self) -> Option<char> {
        self.as_ref().chars().next()
    }

    pub fn chomp(&mut self, len: usize) -> Self {
        let len = std::cmp::min(len, self.len());
        let src = self.src.clone();
        let range = (self.range.0, len);
        self.range.0 += len;
        self.range.1 -= len;
        Self { src, range }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_0_create() {
        let i: Input = "(foo)".into();

        assert_eq!( i.as_ref(), "(foo)" , "as_ref");
        assert_eq!( i.peek_str(5), "(foo)", "peek_5");
        assert_eq!( i.peek_str(4), "(foo", "peek_4");
        assert_eq!( i.peek_str(3), "(fo", "peek_3" );
        assert_eq!( i.len(), 5, "len" );

        assert_eq!(
            i,
            Input {
                src: "(foo)".into(),
                range: (0, 5)
            }
        );
    }

    #[test]
    fn input_1_chomp_0() {
        let mut i: Input = "(foo)".into();
        let j = i.chomp(0);

        assert_eq!( i.as_ref(), "(foo)" );
        assert_eq!( i.peek_str(5), "(foo)" );
        assert_eq!( i.peek_str(4), "(foo" );
        assert_eq!( i.peek_str(3), "(fo" );
        assert_eq!( i.len(), 5 );

        assert_eq!(
            i,
            Input {
                src: "(foo)".into(),
                range: (0, 5)
            }
        );

        assert_eq!( j.as_ref(), "" );
        assert_eq!( j.len(), 0 );
        assert_eq!(
            j,
            Input {
                src: "(foo)".into(),
                range: (0, 0)
            }
        );
    }

    #[test]
    fn input_2_chomp_1() {
        let mut i: Input = "(foo)".into();
        let j = i.chomp(1);

        assert_eq!( i.as_ref(), "foo)" );
        assert_eq!( i.len(), 4 );
        assert_eq!( i.peek_str(4), "foo)" );
        assert_eq!( i.peek_str(3), "foo" );
        assert_eq!( i.peek_str(2), "fo" );

        assert_eq!(
            i,
            Input {
                src: "(foo)".into(),
                range: (1, 4)
            }
        );

        assert_eq!( j.as_ref(), "(" , "j as_ref");
        assert_eq!( j.len(), 1 , "j len");
        assert_eq!( j.peek_str(1), "(" , "j peek 1");

        assert_eq!(
            j,
            Input {
                src: "(foo)".into(),
                range: (0, 1)
            }
        );
    }

    #[test]
    fn input_3_chomp_2() {
        let mut i: Input = "(foo)".into();
        let j = i.chomp(2);

        assert_eq!( i.as_ref(), "oo)" );
        assert_eq!( i.len(), 3 );
        assert_eq!( i.peek_str(3), "oo)" );
        assert_eq!( i.peek_str(2), "oo" );
        assert_eq!( i.peek_str(1), "o" );

        assert_eq!(
            i,
            Input {
                src: "(foo)".into(),
                range: (2, 3)
            }
        );

        assert_eq!( j.as_ref(), "(f" , "j as_ref");
        assert_eq!( j.len(), 2 , "j len");
        assert_eq!( j.peek_str(2), "(f" , "j peek 2");
        assert_eq!( j.peek_str(1), "(" , "j peek 1");

        assert_eq!(
            j,
            Input {
                src: "(foo)".into(),
                range: (0, 2)
            }
        );
    }

    #[test]
    fn input_4_chomp_999() {
        let mut i: Input = "(foo)".into();
        let j = i.chomp(999);

        assert_eq!(i.as_ref(), "");
        assert_eq!(i.len(), 0);
        assert_eq!(i.peek_str(3), "");
        assert_eq!(i.peek_str(2), "");
        assert_eq!(i.peek_str(1), "");

        assert_eq!(
            i,
            Input {
                src: "(foo)".into(),
                range: (5, 0)
            }
        );

        assert_eq!(j.as_ref(), "(foo)", "j as_ref");
        assert_eq!(j.len(), 5, "j len");
        assert_eq!(j.peek_str(2), "(f", "j peek 2");
        assert_eq!(j.peek_str(1), "(", "j peek 1");

        assert_eq!(
            j,
            Input {
                src: "(foo)".into(),
                range: (0, 5)
            }
        );
    }
}
