/// useful functions to calculate the offset between slices and show a hexdump of a slice
pub trait Offset {
    /// offset between the first byte of self and the first byte of the argument
    fn offset(&self, second: &Self) -> usize;
}

impl Offset for [u8] {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();

        snd as usize - fst as usize
    }
}

impl<'a> Offset for &'a [u8] {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();

        snd as usize - fst as usize
    }
}

impl Offset for str {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();

        snd as usize - fst as usize
    }
}

impl<'a> Offset for &'a str {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();

        snd as usize - fst as usize
    }
}
