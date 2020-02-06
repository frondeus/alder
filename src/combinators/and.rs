use crate::*;

macro_rules! impl_parser_tuple {
() => ();
($($name:ident)+) => (
    #[allow(non_snake_case)]
    impl <'a, $($name),* > Parser<'a> for ($($name,)*)
    where
        $($name: Parser<'a>),*
    {
        type Output = ($($name::Output,)*);

        fn parse_state(&self, input: Input<'a>, state: &mut State<'a>) -> (Self::Output, Rest<'a>) {
            let ($($name,)*) = self;
            let i = input;
            $(let ($name, i) = $name.parse_state(i, state);)*
            let o = ($($name,)*);
            (o, i)
        }
    }
);
}

impl_parser_tuple! {AP}
impl_parser_tuple! {AP BP}
impl_parser_tuple! {AP BP CP}
impl_parser_tuple! {AP BP CP DP}
impl_parser_tuple! {AP BP CP DP EP}
impl_parser_tuple! {AP BP CP DP EP FP}
impl_parser_tuple! {AP BP CP DP EP FP GP}
impl_parser_tuple! {AP BP CP DP EP FP GP HP}

macro_rules! impl_into_node_tuple {
() => ();
($($name:ident)+) => (

    #[allow(non_snake_case)]
    impl<'a, $($name),* > Into<NodeVec<'a>> for ($($name,)*)
    where $($name: Into<Node<'a>>),*
    {
        fn into(self) -> NodeVec<'a> {
            let ($($name,)*) = self;
            let mut v = vec![];
            $( v.push($name.into()); )*
            NodeVec(v)
        }
    }
);
}

impl_into_node_tuple! {AP}
impl_into_node_tuple! {AP BP}
impl_into_node_tuple! {AP BP CP}
impl_into_node_tuple! {AP BP CP DP}
impl_into_node_tuple! {AP BP CP DP EP}
impl_into_node_tuple! {AP BP CP DP EP FP}
impl_into_node_tuple! {AP BP CP DP EP FP GP}
impl_into_node_tuple! {AP BP CP DP EP FP GP HP}
