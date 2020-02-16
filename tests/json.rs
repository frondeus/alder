#![feature(drain_filter)]

pub use tmp_alder::*;

mod ast {
    use crate::*;
    use alder_derive::Ast;
    //use std::str::FromStr;
    //use std::collections::HashMap;
    use tmp_alder::Ast;

    #[derive(Debug, Ast)]
    #[cst(parser = "cst::value", node = "cst::Json::Value")]
    enum Value {
        #[cst(tag = "cst::Json::String")]
        String(String),
        #[cst(tag = "cst::Json::Boolean")]
        Boolean(Boolean),
        #[cst(tag = "cst::Json::Array")]
        Array(Array),
        #[cst(tag = "cst::Json::Object")]
        Object(Object),
        #[cst(error)]
        Error(Node)
    }

    impl Value {
        fn name(&self) -> &str {
            match self {
                Value::String(_) => "string",
                Value::Boolean(_) => "boolean",
                Value::Array(_) => "array",
                Value::Object(_) => "object",
                Value::Error(_) => "error"
            }
        }
        fn as_string(&self) -> &String {
            match self {
                Value::String(s) => s,
                _ => panic!("Expected string")
            }
        }

        fn as_boolean(&self) -> &Boolean {
            match self {
                Value::Boolean(b) => b,
                _ => panic!("Expected boolean")
            }
        }

        fn as_array(&self) -> &Array {
            match self {
                Value::Array(a) => a,
                v => panic!("Expected array, found: {}", v.name())
            }
        }

        fn as_object(&self) -> &Object {
            match self {
                Value::Object(o) => o,
                v => panic!("Expected object, found: {}", v.name())
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Ast)]
    #[cst(node = "cst::Json::String")]
    struct String{ node: Node }

    impl String {
        fn value(&self) -> &str {
            let val = self.node.children.iter()
                .filter(|c| c.is(cst::Json::Value)).next()
                .map(|value| value.span.as_ref())
                .unwrap_or_default();
            val
        }
    }

    #[derive(Debug, Ast)]
    #[cst(node = "cst::Json::Boolean")]
    struct Boolean{ node: Node }

    impl Boolean {
        fn value(&self) -> bool {
            self.node.span.as_ref() == "true"
        }
    }

    #[derive(Debug, Ast)]
    #[cst(node = "cst::Json::Array")]
    struct Array {
        node: Node,
        children: Vec<Value>,
    }

    impl Array {
        fn iter(&self) -> impl Iterator<Item = &Value> {
            self.children.iter()
        }
    }

    #[derive(Debug, Ast)]
    #[cst(node = "cst::Json::Object")]
    struct Object {
        #[cst(flatten)]
        pairs: Vec<KeyValuePair>,
        node: Node
    }

    impl Object {
        fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
            self.pairs.iter()
                .map(|KeyValuePair{key, value}| (key.value(), value))
        }
    }

    #[derive(Debug, Ast)]
    struct KeyValuePair {
        #[cst(find = "cst::Json::Key")]
        key: String,
        #[cst(find = "cst::Json::Value")]
        value: Value
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn string() {
            let value = Value::from_str(r#""true""#).unwrap();
            let v = value.as_string();
            assert_eq!("true", v.value());
        }

        #[test]
        fn boolean() {
            let value = Value::from_str(r#"true"#).unwrap();
            let v = value.as_boolean();
            assert_eq!(true, v.value());

            let value = Value::from_str(r#"false"#).unwrap();
            let v = value.as_boolean();
            assert_eq!(false, v.value());
        }

        #[test]
        fn array() {
            let value = Value::from_str("[true,false]").unwrap();
            let v = value.as_array();
            let values = v.iter()
                .map(|v| v.as_boolean())
                .map(|v| v.value())
                .collect::<Vec<_>>();

            assert_eq!(vec![true, false], values);
        }

        #[test]
        fn object() {
            let value = Value::from_str(r#"{ "a": true, "b": false }"#).unwrap();
            let v = value.as_object();
            let values = v.iter()
                .map(|(k, v)| {
                    (k, v.as_boolean().value())
                }).collect::<Vec<_>>();

            assert_eq!(values, vec![("a", true), ("b", false)]);
        }

        #[test]
        fn adv_object() {
            let value = Value::from_str(r#"{ "a": true, "c": [true], "b": false }"#).unwrap();
            let v = value.as_object();
            let values = v.iter()
                .map(|(k, v)| {
                    (k, v.name())
                }).collect::<Vec<_>>();

            assert_eq!(values, vec![("a", "boolean"), ("c", "array"), ("b", "boolean")]);
        }

        #[test]
        fn err_object() {
            let value = Value::from_str(r#"{ "a": true, c": [true], "b": false }"#).unwrap();
            let v = value.as_object();
            let values = v.iter()
                .map(|(k, v)| {
                    (k, v.name())
                }).collect::<Vec<_>>();

            assert_eq!(values, vec![("a", "boolean"), ("", "array"), ("b", "boolean")]);
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

    use derive_more::Display;
    #[derive(Debug, Display, Clone)]
    enum Problem {
        #[display(fmt = "Expected `true` or `false` but found:")]
        InvalidBoolean,

        #[display(fmt = "Expected `,` or `]` but found:")]
        InvalidTokenArray,

        #[display(fmt = "Expected `true`, `false`, `[`, `{{` or `\"` but found:")]
        InvalidTokenValue,
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
    /// [trua, falsa]
    /// [truadsadsa, falsa]
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
                    _ => state.add(raise(Problem::InvalidTokenValue, 1)),
                    //c => todo!("value {:?}", c),
                };
            }),
        )
    }

    /// ""
    /// "foo"
    /// "foo bar"
    /**
        "foo
        bar"
    */
    #[alder]
    pub fn string() -> impl Parser {
        no_extra(node(Json::String, |state| {
            state.add(token('"'));
            state.add(chomp_while(Json::Value, |c| c != '"'));
            state.add(token('"'));
        }))
    }

    /// true
    /// false
    /// dupa
    /// tdupa
    #[alder]
    fn boolean() -> impl Parser {
        v_node(Json::Boolean, |state| match state.input.peek() {
            Some('t') => state.add(tag("true")),
            Some('f') => state.add(tag("false")),
            _ => state.add(raise(Problem::InvalidBoolean, 5)),
                //todo!("boolean {:?}", c),
        })
    }

    /// {}
    /// {"foo":"bar"}
    /// { "foo": "bar" }
    /// { "foo": true, "bar": false }
    /// { "foo": true, "bar": {} }
    /// { "foo": true, "bar": { "foo": false } }
    /// { "foo": true, "bar": [] }
    #[alder]
    fn object() -> impl Parser {
        with_extra(extra(),
        node(Json::Object, |state| {
            state.add(token('{'));
            match state.input.peek() {
                Some('}') => (),
                _ => loop {
                    state.add(field(Json::Key, string()));
                    state.add(fuse(token(':')));
                    state.add(value());
                    match state.input.peek() {
                        Some('}') => {
                            break;
                        }
                        Some(',') => {
                            state.add(fuse(token(',')));
                        },
                        c => todo!("object {:?}", c),
                    };
                },
            };
            state.add(fuse(token('}')));
        }))
    }

    /// []
    /// [true]
    /// [true,false]
    /// [ ]
    /// [ true ]
    /// [ true, false ]
    /// [ true, false, ]
    #[alder]
    fn array() -> impl Parser {
        with_extra(extra(), node(Json::Array, |state| {
            state.add(token('['));
            match state.input.peek() {
                Some(']') => (),
                _ => loop {
                    state.add(value());
                    match state.input.peek() {
                        Some(']') => {
                            break;
                        }
                        Some(',') => {
                            state.add(fuse(token(',')));
                            if let Some(']') = state.input.peek() { // Trailing comma
                                break;
                            }
                        },
                        _ => state.add(raise(Problem::InvalidTokenArray, 1)),
                        //c => todo!("array {:?}", c),
                    };
                },
            }
            state.add(fuse(token(']')));
        }))
    }
}
