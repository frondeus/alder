use crate::*;

pub trait Ast: Sized {
    fn parse(iter: &mut impl Iterator<Item = Node>) -> Option<Self>;
}

impl<T> Ast for Vec<T> where T: Ast {
    fn parse(iter: &mut impl Iterator<Item=Node>) -> Option<Self> {
        let res: Self = iter.filter_map(|node| {
            let nodes = node.iter().cloned().collect::<Vec<Node>>();
            Ast::parse(&mut nodes.into_iter())
        }).collect();

        Some(res)
    }
}

