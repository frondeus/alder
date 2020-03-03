use crate::*;
use std::iter::FromIterator;

pub trait Ast: Sized {
    fn parse(iter: &mut impl Iterator<Item = Node>) -> Option<Self>;
}

impl<T> Ast for Vec<T>
where
    T: Ast,
{
    fn parse(iter: &mut impl Iterator<Item = Node>) -> Option<Self> {
        let res: Self = iter
            .filter_map(|node| {
                let nodes = node.iter().cloned().collect::<Vec<Node>>();
                Ast::parse(&mut nodes.into_iter())
            })
            .collect();

        Some(res)
    }
}

impl<T> Ast for Box<T>
where
    T: Ast,
{
    fn parse(iter: &mut impl Iterator<Item = Node>) -> Option<Self> {
        T::parse(iter).map(Box::new)
    }
}

pub trait FromCst {
    fn from_node(node: &Node) -> Option<Self>
    where
        Self: Sized;
}

pub trait CstIterExt<'a>: Iterator<Item = &'a Node> + Clone {
    fn find_cst<T>(&mut self) -> Option<T>
    where
        T: FromCst,
        Self: Sized,
    {
        self.find_map(|n| FromCst::from_node(n))
    }

    fn collect_cst<T, E>(&mut self) -> E
    where
        E: FromIterator<T>,
        T: FromCst,
        Self: Sized,
    {
        self.clone().filter_map(|n| FromCst::from_node(n)).collect()
    }
}
impl<'a, T> CstIterExt<'a> for T where T: Iterator<Item = &'a Node> + Clone {}
