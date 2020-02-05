use crate::*;

impl<'a> From<Node<'a>> for NodeVec<'a> {
    fn from(n: Node<'a>) -> Self {
        Self(vec![n])
    }
}

pub fn node<'a, T: Into<NodeVec<'a>>>(
    node_kind: NodeKind,
    parser: impl Parser<'a, Output = T>,
) -> impl Parser<'a, Output = Node<'a>> {
    move |i: Input<'a>, state: &mut State<'a>| {
        let (output, rest) = parser.parse_state(i, state);

        let children: NodeVec = output.into();

        let index = i.offset(rest); //Recognize
        let node = Node {
            kind: node_kind,
            location: &i[..index],
            children: children.0,
        };

        (node, rest)
    }
}
