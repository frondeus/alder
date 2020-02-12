pub use tmp_alder::*;

mod ast {
    use crate::*;

    #[derive(Debug)]
    enum Value {
        String(String),
        Boolean(Boolean),
        Array(Array),
        Object(Object),
    }

    impl Value {
        fn from(node: Node) -> Self {
            if node.is(cst::Json::Boolean) {
                Value::Boolean(Boolean(node))
            }
            else if node.is(cst::Json::String) {
                Value::String(String(node))
            }
            else if node.is(cst::Json::Array) {
                Value::Array(Array(node))
            }
            else if node.is(cst::Json::Object) {
                Value::Object(Object(node))
            }
            else {
                todo!()
            }
        }

        fn from_str(input: &str) -> Self {
            use super::cst;
            let mut parsed = State::parse(input, cst::value());
            let node = parsed.nodes.pop().unwrap();

            Self::from(node)
        }

        fn unwrap_string(self) -> String {
            match self {
                Value::String(s) => s,
                _ => panic!("Expected string")
            }
        }

        fn unwrap_boolean(self) -> Boolean {
            match self {
                Value::Boolean(b) => b,
                _ => panic!("Expected boolean")
            }
        }

        fn unwrap_array(self) -> Array {
            match self {
                Value::Array(a) => a,
                _ => panic!("Expected array")
            }
        }
    }

    #[derive(Debug)]
    struct String(Node);

    impl String {
        fn value(&self) -> &str {
            let val = self.0.children.iter().filter(|c| c.is(cst::Json::Value)).next()
                .map(|value| value.span.as_ref())
                .unwrap_or_default();
            val
        }
    }

    #[derive(Debug)]
    struct Boolean(Node);

    impl Boolean {
        fn value(&self) -> bool {
            self.0.span.as_ref() == "true"
        }
    }

    #[derive(Debug)]
    struct Array(Node);

    impl Array {
        fn into_iter(self) -> impl Iterator<Item = Value> {
            self.0.children.into_iter()
                .filter(|n| n.is(cst::Json::Value))
                .map(|v| Value::from(v))
        }
    }

    #[derive(Debug)]
    struct Object(Node);

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn string() {
            let value = Value::from_str(r#""true""#);
            let v = value.unwrap_string();
            assert_eq!("true", v.value());
        }

        #[test]
        fn boolean() {
            let value = Value::from_str(r#"true"#);
            let v = value.unwrap_boolean();
            assert_eq!(true, v.value());

            let value = Value::from_str(r#"false"#);
            let v = value.unwrap_boolean();
            assert_eq!(false, v.value());
        }

        #[test]
        fn array() {
            let value = Value::from_str("[true,false]");
            let v = value.unwrap_array();
            let values = v.into_iter()
                .map(|v| v.unwrap_boolean())
                .map(|v| v.value())
                .collect::<Vec<_>>();

            assert_eq!(vec![true, false], values);
        }
    }
}

mod cst {
    use alder::*;
    use alder_derive::alder;
    use tmp_alder as alder;

    node_ids! {
        pub Json:
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
    pub fn value() -> impl Parser {
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
        v_node(Json::Boolean, |state| match state.input.peek() {
            Some('t') => state.add(tag("true")),
            Some('f') => state.add(tag("false")),
            c => todo!("boolean {:?}", c),
        })
    }

    fn object() -> impl Parser {
        node(Json::Object, |state| {
            state.add(token('{'));
            match state.input.peek() {
                Some('}') => (),
                _ => loop {
                    state.add(field(Json::Key, string()));
                    state.add(token(':'));
                    state.add(value());
                    match state.input.peek() {
                        Some('}') => {
                            break;
                        }
                        Some(',') => state.add(token(',')),
                        c => todo!("object {:?}", c),
                    };
                },
            };
            state.add(token('}'));
        })
    }

    fn array() -> impl Parser {
        node(Json::Array, |state| {
            state.add(token('['));
            match state.input.peek() {
                Some(']') => (),
                _ => loop {
                    state.add(value());
                    match state.input.peek() {
                        Some(']') => {
                            break;
                        }
                        Some(',') => state.add(token(',')),
                        c => todo!("array {:?}", c),
                    };
                },
            }
            state.add(token(']'));
        })
    }
}
