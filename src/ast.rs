use crate::*;

pub trait Ast: Sized {
    fn parse(iter: &mut impl Iterator<Item = Node>) -> Option<Self>;
}

impl<T> Ast for Vec<T> where T: Ast {
    fn parse(mut iter: &mut impl Iterator<Item=Node>) -> Option<Self> {
        /*
        let mut c: Vec<T> = Default::default();
        loop {
            if let Some(item) = Ast::parse(&mut iter) {
                c.push(item);
            }
            else { return Some(c); }
        }
        */
        let res: Self = iter.filter_map(|node| {
            let nodes = node.iter().cloned().collect::<Vec<Node>>();
            Ast::parse(&mut nodes.into_iter())
        }).collect();

        Some(res)
    }
}

