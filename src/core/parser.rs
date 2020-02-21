use crate::*;

pub trait Parser<T = Node> {
    fn parse(&self, state: &mut State) -> T;

    fn map<F>(self, f: F) -> Map<Self, F, T>
        where Self: Sized {
        Map::new(self, f)
    }

    fn boxed<'a>(self) -> Box<dyn Parser<T> + 'a>
    where
        Self: 'a + Sized,
    {
        Box::new(self)
    }

    fn arc<'a>(self) -> std::sync::Arc<dyn Parser<T> + 'a>
    where
        Self: 'a + Sized,
    {
        std::sync::Arc::new(self)
    }
}

impl<F, T> Parser<T> for F
where
    F: Fn(&mut State) -> T,
{
    fn parse(&self, state: &mut State) -> T {
        self(state)
    }
}
