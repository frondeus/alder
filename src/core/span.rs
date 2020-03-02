use crate::*;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub struct Span {
    src: Arc<str>,
    pub(crate) range: (usize, usize), // pos, len
}

#[cfg(test)]
impl Span {
    pub fn test(src: &'static str, range: (usize, usize)) -> Self {
        let src: Arc<str> = Arc::from(src);
        Self { src, range }
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)?;
        write!(f, "{:?}", &self.range)?;
        Ok(())
    }
}

impl<'a> From<&'a str> for Span {
    fn from(input: &'a str) -> Span {
        let src: Arc<str> = Arc::from(input);
        let range = (0, src.len());
        Self { src, range }
    }
}

impl Offset for Span {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.range.0;
        let snd = second.range.0;

        snd as usize - fst as usize
    }
}

impl AsRef<str> for Span {
    fn as_ref(&self) -> &str {
        &self.src[self.range.0..self.range.0 + self.range.1]
    }
}

use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};
impl Span {
    pub fn graphemes_idx(&self) -> GraphemeIndices {
        self.as_ref().grapheme_indices(true)
    }
}

impl Span {
    pub fn full(&self) -> Span {
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

    /// Try use utf8 oriented parsers instead
    pub(crate) fn chomp_chars(&mut self, len: usize) -> Self {
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
        let i: Span = "(foo)".into();

        assert_eq!(i.as_ref(), "(foo)", "as_ref");
        assert_eq!(i.len(), 5, "len");

        assert_eq!(
            i,
            Span {
                src: "(foo)".into(),
                range: (0, 5)
            }
        );
    }

    #[test]
    fn input_1_chomp_0() {
        let mut i: Span = "(foo)".into();
        let j = i.chomp_chars(0);

        assert_eq!(i.as_ref(), "(foo)");
        assert_eq!(i.len(), 5);

        assert_eq!(
            i,
            Span {
                src: "(foo)".into(),
                range: (0, 5)
            }
        );

        assert_eq!(j.as_ref(), "");
        assert_eq!(j.len(), 0);
        assert_eq!(
            j,
            Span {
                src: "(foo)".into(),
                range: (0, 0)
            }
        );
    }

    #[test]
    fn input_2_chomp_1() {
        let mut i: Span = "(foo)".into();
        let j = i.chomp_chars(1);

        assert_eq!(i.as_ref(), "foo)");
        assert_eq!(i.len(), 4);

        assert_eq!(
            i,
            Span {
                src: "(foo)".into(),
                range: (1, 4)
            }
        );

        assert_eq!(j.as_ref(), "(", "j as_ref");
        assert_eq!(j.len(), 1, "j len");

        assert_eq!(
            j,
            Span {
                src: "(foo)".into(),
                range: (0, 1)
            }
        );
    }

    #[test]
    fn input_3_chomp_2() {
        let mut i: Span = "(foo)".into();
        let j = i.chomp_chars(2);

        assert_eq!(i.as_ref(), "oo)");
        assert_eq!(i.len(), 3);

        assert_eq!(
            i,
            Span {
                src: "(foo)".into(),
                range: (2, 3)
            }
        );

        assert_eq!(j.as_ref(), "(f", "j as_ref");
        assert_eq!(j.len(), 2, "j len");

        assert_eq!(
            j,
            Span {
                src: "(foo)".into(),
                range: (0, 2)
            }
        );
    }

    #[test]
    fn input_4_chomp_999() {
        let mut i: Span = "(foo)".into();
        let j = i.chomp_chars(999);

        assert_eq!(i.as_ref(), "");
        assert_eq!(i.len(), 0);

        assert_eq!(
            i,
            Span {
                src: "(foo)".into(),
                range: (5, 0)
            }
        );

        assert_eq!(j.as_ref(), "(foo)", "j as_ref");
        assert_eq!(j.len(), 5, "j len");

        assert_eq!(
            j,
            Span {
                src: "(foo)".into(),
                range: (0, 5)
            }
        );
    }
}
