use crate::*;

pub trait Parser {
    fn parse(&self, state: &mut State) -> Node;

    fn boxed<'a>(self) -> Box<dyn Parser + 'a>
    where
        Self: 'a + Sized,
    {
        Box::new(self)
    }

    fn arc<'a>(self) -> std::sync::Arc<dyn Parser + 'a>
    where
        Self: 'a + Sized,
    {
        std::sync::Arc::new(self)
    }
}

impl<F> Parser for F
where
    F: Fn(&mut State) -> Node,
{
    fn parse(&self, state: &mut State) -> Node {
        self(state)
    }
}
