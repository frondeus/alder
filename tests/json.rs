pub use tmp_alder::*;

mod ast {
    use crate::*;

    struct Value {
        node: Node,
        kind: ValueKind
    }

    enum ValueKind {
        String(String),
        Boolean(Boolean),
        Array(Array),
        Object(Object)
    }

    struct String(Node);

    struct Boolean(Node);

    struct Array(Node);

    struct Object(Node);

    #[cfg(test)]
    mod tests {
        use super::*;

    }

}

mod cst {
    use tmp_alder as alder;
    use alder::*;
    use alder_derive::alder;

    node_ids! {
        Json:
            // Extra
            WS,

            // Nodes
            String,
            Value,
            Boolean,
            Array,
            Object,

            // Fields
            Key
    }

    fn extra() -> std::sync::Arc<dyn Parser> {
        chomp_while(Json::WS, |c| c.is_whitespace()).arc()
    }

    fn string() -> impl Parser {
        no_extra(node(Json::String, |state| {
            state.add(token('"'));
            state.add(chomp_while(Json::Value, |c| c != '"'));
            state.add(token('"'));
        }))
    }

    /// true
    /// false
    #[alder]
    fn boolean() -> impl Parser {
        v_node(Json::Boolean, |state| {
            match state.input.peek() {
                Some('t') => state.add(tag("true")),
                Some('f') => state.add(tag("false")),
                c => todo!("boolean {:?}", c),
            }
        })
    }

    /// []
    /// [true]
    /// [[true]]
    /// [true,false]
    /// [ true , false ]
    /// ["foo"]
    /// ["  foo   "]
    /**
        {
            "true": false,
            "false": true
        }
    */
    #[alder]
    fn value() -> impl Parser {
        with_extra(
            extra(),
            v_node(Json::Value, |state| {
                match state.input.peek() {
                    Some('t') | Some('f') => state.add(boolean()),
                    Some('[') => state.add(array()),
                    Some('{') => state.add(object()),
                    Some('"') => state.add(string()),
                    c => todo!("value {:?}", c),
                };
            }),
        )
    }

    fn object() -> impl Parser {
        node(Json::Object, |state| {
            state.add(token('{'));
            match state.input.peek() {
                Some('}') => (),
                _ => {
                    loop {
                        state.add(field(Json::Key, string()));
                        state.add(token(':'));
                        state.add(value());
                        match state.input.peek() {
                            Some('}') => { break; }
                            Some(',') => state.add(token(',')),
                            c => todo!("object {:?}", c),
                        };
                    }
                }
            };
            state.add(token('}'));
        })
    }

    fn array() -> impl Parser {
        node(Json::Array, |state| {
            state.add(token('['));
            match state.input.peek() {
                Some(']') => (),
                _ => {
                    loop {
                        state.add(value());
                        match state.input.peek() {
                            Some(']') => { break; }
                            Some(',') => state.add(token(',')),
                            c => todo!("array {:?}", c),
                        };
                    }
                }
            }
            state.add(token(']'));
        })
    }
}
