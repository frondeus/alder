use crate::*;

pub trait Alt<'a> {
    type Output;
    fn alt(&self, i: Input<'a>, state: &mut State<'a>) -> (Self::Output, Rest<'a>);
}

pub struct Or<Tuple> {
    tuple: Tuple,
}

pub trait IntoParser {
    fn or(self) -> Or<Self>
        where
            Self: Sized;
}

impl<'a, Tuple> Parser<'a> for Or<Tuple>
    where
        Tuple: Alt<'a>,
{
    type Output = Tuple::Output;

    fn parse_state(&self, i: Input<'a>, state: &mut State<'a>) -> (Self::Output, Rest<'a>) {
        self.tuple.alt(i, state)
    }
}

pub fn or<'a, P1, P2, Output>(p1: P1, p2: P2) -> impl Parser<'a, Output = Output>
    where
        P1: Parser<'a, Output = Output>,
        P2: Parser<'a, Output = Output>,
{
    (p1, p2).or()
}

macro_rules! impl_alt {
    () => ();
    ($($name:ident)+) => (
        impl<'a, $($name),*> IntoParser for ($($name,)*) {
            fn or(self) -> Or<Self> where Self: Sized { Or { tuple: self } }
        }

        #[allow(non_snake_case, unused_variables)]
        impl <'a, Output, $($name),* > Alt<'a> for ($($name,)*)
        where
            $($name: Parser<'a, Output = Output>),*
        {
            type Output = Output;

            fn alt(&self, i: Input<'a>, state: &mut State<'a>) -> (Self::Output, Rest<'a>) {
                let ($($name,)*) = self;

                //TODO: How to implement backtracking (if necessary?)
                //let before = state.clone();

                $(
                    let (o, r) = $name.parse_state(i, state);
                )*

                (o, r)
            }
        }
    );
}

impl_alt! {AP BP}
impl_alt! {AP BP CP}
impl_alt! {AP BP CP DP}
impl_alt! {AP BP CP DP EP}
impl_alt! {AP BP CP DP EP FP}
impl_alt! {AP BP CP DP EP FP GP}
impl_alt! {AP BP CP DP EP FP GP HP}

