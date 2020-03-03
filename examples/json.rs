#![allow(dead_code)]

// First we create the lossless parser
#[rustfmt::skip]
mod cst {
    use alder::*;

    #[cfg(not(feature = "derive"))]
    use alder_derive::alder_test;

    // Handy macro to create enum-like structure with static string constants.
    node_ids! {
        pub Json:
            // Extra
            WS,
            Comment,
            InlineComment,
            MultilineComment,

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
        #[display(fmt = "I expected `true` or `false`")]
        InvalidBoolean,

        #[display(fmt = "I expected `,` or `]`")]
        InvalidTokenArray,

        #[display(fmt = "I expected `,` or `}}`")]
        InvalidTokenObject,

        #[display(fmt = "I expected `true`, `false`, `[`, `{{` or `\"`")]
        InvalidTokenValue,

        #[display(fmt = "I expected `*/`")]
        UnexpectedEOFComment,

        #[display(fmt = "I expected `//` or `/*`")]
        InvalidTokenComment,
    }

    fn extra() -> std::sync::Arc<dyn Parser> {
        v_node(NodeId::EXTRA, |state| {
            while !state.panic {
                match state.peek(1).as_ref() {
                    "" => break,
                    s if s.is_ws() => state.add(ws()),
                    "/" => state.add(comment()),
                    _ => break,
                }
            }
        })
        .arc()
    }

    fn ws() -> impl Parser {
        recognize(Json::WS, chomp_while(is_ws))
    }

    /// // fooo
    /**
        // fooo
        rest
    */
    /**
        /* foo
        bar
        baz */
    */
    /// /* foo
    #[alder_test]
    fn comment() -> impl Parser {
        v_node(Json::Comment,
               |state| match state.peek(2).as_ref() {
                   "//" => state.add(recognize(Json::InlineComment, chomp_until(is_line_ending))),
                   "/*" => state.add(multiline_comment()),
                   _ => state.add(raise(Problem::InvalidTokenComment, 1)),
               }
        )
    }

    fn multiline_comment() -> impl Parser {
        node(Json::MultilineComment, |state| loop {
            match state.peek(2).as_ref() {
                "" => {
                    state.add(raise(Problem::UnexpectedEOFComment, 0));
                    break;
                },
                "*/" => {
                    state.chomp(2);
                    break;
                },
                _ => {
                    state.chomp(1);
                }
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
    /// [trua, falsa]
    /// [truadsadsa, falsa]
    /**
        {
            "a": "foo,
            "b": "bar",
            "c": "baz"
        }
    */
    /**
        {
            "a": "foo", // Here
            "b": /* Here */ "bar",
            /*
                HERE
                */
            "c": "baz"
        }
    */
    /**
        [true, / invalid comment
        false]
    */
    #[alder_test]
    pub fn value() -> impl Parser {
        with_extra(
            extra(),
            v_node(Json::Value, |state| {
                match state.peek(1).as_ref() {
                    "t" | "f" => state.add(boolean()),
                    "[" => state.add(array()),
                    "{" => state.add(object()),
                    "\"" => state.add(string()),
                    _ => state.add(raise(Problem::InvalidTokenValue, 1)),
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
    #[alder_test]
    pub fn string() -> impl Parser {
        no_extra(node(Json::String, |state| {
            state.add("\"");
            state.add(recognize(Json::Value,chomp_until( |c| c == "\"" || c.is_line_ending())));
            state.add("\"");
        }))
    }

    /// true
    /// false
    /// dupa
    /// tdupa
    #[alder_test]
    fn boolean() -> impl Parser {
        v_node(Json::Boolean, |state| match state.peek(1).as_ref() {
            "t" => state.add("true"),
            "f" => state.add("false"),
            _ => state.add(raise(Problem::InvalidBoolean, 1)),
        })
    }

    /// {}
    /// {"foo":"bar"}
    /// { "foo": "bar" }
    /// { "foo": true, "bar": false }
    /// { "foo": true, "bar": {} }
    /// { "foo": true, "bar": { "foo": false } }
    /// { "foo": true, "bar": [] }
    /// { "foo": true
    /// { "foo": truadsadsadssa, "bar": false }
    #[alder_test]
    fn object() -> impl Parser {
        with_extra(
            extra(),
            node(Json::Object, |state| {
                state.add("{");
                match state.peek(1).as_ref() {
                    "}" => (),
                    _ => 'outer: loop {
                        state.add(field(Json::Key, string()));
                        state.add(recover(":"));
                        state.add(value());
                        'inner: loop {
                            match state.peek(1).as_ref() {
                                "}" => {
                                    break 'outer;
                                }
                                "," => {
                                    state.add(recover(","));
                                    if let "}" = state.peek(1).as_ref() {
                                        // Trailing comma
                                        break 'outer;
                                    }
                                    break 'inner;
                                }
                                "" => { // EOF
                                    state.add(raise(Problem::InvalidTokenObject, 1));
                                    break 'outer;
                                }
                                _ => state.add(raise(Problem::InvalidTokenObject, 1)),
                            };
                        }
                    },
                };
                state.add(recover("}"));
            }),
        )
    }

    /// []
    /// [true]
    /// [true,false]
    /// [ ]
    /// [ true ]
    /// [ true, false ]
    /// [ true, false, ]
    /// [trua, falsa]
    /// [truadsadsa, falsa]
    /// [true, false
    /// [truad  sadsa, falsa]
    #[alder_test]
    fn array() -> impl Parser {
        with_extra(
            extra(),
            node(Json::Array, |state| {
                state.add("[");
                match state.peek(1).as_ref() {
                    "]" => (),
                    _ => 'outer: loop {
                        state.add(value());
                        'inner: loop {
                            // Until we find either ']' or ','
                            match state.peek(1).as_ref() {
                                "]" => {
                                    break 'outer;
                                }
                                "," => {
                                    state.add(recover(","));
                                    if let "]" = state.peek(1).as_ref() {
                                        // Trailing comma
                                        break 'outer;
                                    }
                                    break 'inner;
                                }
                                "" => { // EOF
                                    state.add(raise(Problem::InvalidTokenArray, 1));
                                    break 'outer;
                                }
                                _ => state.add(raise(Problem::InvalidTokenArray, 1)),
                            };
                        }
                    },
                }
                state.add(recover("]"));
            }),
        )
    }
}

mod ast {
    use crate::*;
    use alder::{Ast, Node, State};
    #[cfg(not(feature = "derive"))]
    use alder_derive::Ast;

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
        Error(Node),
    }

    impl Value {
        fn name(&self) -> &str {
            match self {
                Value::String(_) => "string",
                Value::Boolean(_) => "boolean",
                Value::Array(_) => "array",
                Value::Object(_) => "object",
                Value::Error(_) => "error",
            }
        }
        fn as_string(&self) -> &String {
            match self {
                Value::String(s) => s,
                _ => panic!("Expected string"),
            }
        }

        fn as_boolean(&self) -> &Boolean {
            match self {
                Value::Boolean(b) => b,
                _ => panic!("Expected boolean"),
            }
        }

        fn as_array(&self) -> &Array {
            match self {
                Value::Array(a) => a,
                v => panic!("Expected array, found: {}", v.name()),
            }
        }

        fn as_object(&self) -> &Object {
            match self {
                Value::Object(o) => o,
                v => panic!("Expected object, found: {}", v.name()),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Ast)]
    #[cst(node = "cst::Json::String")]
    struct String {
        node: Node,
    }

    impl String {
        fn value(&self) -> &str {
            self.node
                .children
                .iter()
                .find(|c| c.is(cst::Json::Value))
                .map(|value| value.span.as_ref())
                .unwrap_or_default()
        }
    }

    #[derive(Debug, Ast)]
    #[cst(node = "cst::Json::Boolean")]
    struct Boolean {
        node: Node,
    }

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
        node: Node,
    }

    impl Object {
        fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
            self.pairs
                .iter()
                .map(|KeyValuePair { key, value }| (key.value(), value))
        }
    }

    #[derive(Debug, Ast)]
    struct KeyValuePair {
        #[cst(find = "cst::Json::Key")]
        key: String,
        #[cst(find = "cst::Json::Value")]
        value: Value,
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
            let values = v
                .iter()
                .map(|v| v.as_boolean())
                .map(|v| v.value())
                .collect::<Vec<_>>();

            assert_eq!(vec![true, false], values);
        }

        #[test]
        fn object() {
            let value = Value::from_str(r#"{ "a": true, "b": false }"#).unwrap();
            let v = value.as_object();
            let values = v
                .iter()
                .map(|(k, v)| (k, v.as_boolean().value()))
                .collect::<Vec<_>>();

            assert_eq!(values, vec![("a", true), ("b", false)]);
        }

        #[test]
        fn adv_object() {
            let value = Value::from_str(r#"{ "a": true, "c": [true], "b": false }"#).unwrap();
            let v = value.as_object();
            let values = v.iter().map(|(k, v)| (k, v.name())).collect::<Vec<_>>();

            assert_eq!(
                values,
                vec![("a", "boolean"), ("c", "array"), ("b", "boolean")]
            );
        }

        #[test]
        fn err_object() {
            let value = Value::from_str(r#"{ "a": true, c": [true], "b": false }"#).unwrap();
            let v = value.as_object();
            let values = v.iter().map(|(k, v)| (k, v.name())).collect::<Vec<_>>();

            assert_eq!(
                values,
                vec![("a", "boolean"), ("", "array"), ("b", "boolean")]
            );
        }
    }
}

fn main() {}
