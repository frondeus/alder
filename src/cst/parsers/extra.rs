use crate::*;

pub struct WithExtra<P: Parser> {
    extra: std::sync::Arc<dyn Parser>,
    parser: P,
}

impl<P: Parser> Parser for WithExtra<P> {
    fn parse(&self, state: &mut State) -> Node {
        let extra = self.extra.clone();
        state.push_extra(extra);
        let node = self.parser.parse(state);
        state.pop_extra();
        node
    }
}

pub fn with_extra<P: Parser>(extra: std::sync::Arc<dyn Parser>, parser: P) -> WithExtra<P> {
    WithExtra { extra, parser }
}

pub fn no_extra(parser: impl Parser) -> impl Parser {
    move |state: &mut State| {
        state.push_atomic();
        let node = parser.parse(state);
        state.pop_extra();
        node
    }
}
