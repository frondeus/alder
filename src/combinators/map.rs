use crate::*;

pub struct Map<Inner, Func> {
    pub(crate) p: Inner,
    pub(crate) f: Func,
}

impl<'a, Inner, Func, Output> Parser<'a> for Map<Inner, Func>
where
    Inner: Parser<'a>,
    Func: Fn(Inner::Output) -> Output,
{
    type Output = Output;

    fn parse_state(&self, i: Input<'a>, state: &mut State<'a>) -> (Self::Output, Rest<'a>) {
        let f = &self.f;
        let (p1, i) = self.p.parse_state(i, state);
        let p2 = f(p1);
        let t = p2;
        (t, i)
    }
}
