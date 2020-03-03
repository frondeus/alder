#![allow(dead_code)]

// First we create the lossless parser
#[rustfmt::skip]
#[cfg(feature = "derive")]
mod cst {
    use alder::*;

    // Handy macro to create enum-like structure with static string constants.
    node_ids! {
        pub Calc:
            // Extra
            WS,

            // Nodes
            Value,
            Parent,
            Binary,
            Unary,
            Number
    }

    use derive_more::Display;
    #[derive(Debug, Display, Clone)]
    enum Problem {
        //TODO:
        #[display(fmt = "Todo")]
        Todo,
    }

    fn extra() -> std::sync::Arc<dyn Parser> {
        recognize(Calc::WS, chomp_while( is_ws)).arc()
    }

    /// 4
    /// 2 + 3
    /// 2 + 3 * 4
    /// -2
    /// (2 + 3) * 4
    /// -(2 + 3)
    #[alder_test]
    pub fn value() -> impl Parser {
        with_extra(
            extra(),
            infix(Calc::Binary, left_value(), |op| match op {
                "*" => Some((20, "*")),
                "/" => Some((20, "/")),
                "+" => Some((10, "+")),
                "-" => Some((10, "-")),
                _ => None
            }).map(|node: Node| node.with_alias(Calc::Value))
        )
    }

    fn left_value() -> impl Parser + Clone {
        |state: &mut State| v_node(Calc::Value, |state| {
            match state.peek(1).as_ref() {
                s if s.is_digits() => state.add(number()),
                "(" => {
                    state.add("(");
                    state.add(value());
                    state.add(")");
                },
                "-" => {
                    state.add(minus());
                },
                _ => state.add(raise(Problem::Todo, 1)),
            };
        }).parse(state)
    }

    fn minus() -> impl Parser {
        node(Calc::Unary, |state| {
            state.add("-");
            state.add(value());
        })
    }

    fn number() -> impl Parser {
        recognize(Calc::Number, chomp_while(is_digits))
    }

}

#[cfg(feature = "derive")]
mod ast {
    use crate::*;
    use alder::{Ast, Node, NodeId, Span, State};

    #[derive(Debug, Ast)]
    #[cst(
        parser = "cst::value",
        node = "cst::Calc::Value",
        skip = "NodeId::TOKEN"
    )]
    pub enum Value {
        #[cst(tag = "cst::Calc::Number")]
        Number(Number),
        #[cst(tag = "cst::Calc::Unary")]
        Unary(Unary),
        #[cst(tag = "cst::Calc::Binary")]
        Binary(Binary),
        #[cst(error)]
        Error(Node),
    }

    #[derive(Debug, Ast)]
    pub struct Number {
        span: Span,
    }

    #[derive(Debug, Ast)]
    pub struct Unary {
        #[cst(find = "NodeId::TOKEN")]
        op: UnOp,
        #[cst(find = "cst::Calc::Value")]
        right: Box<Value>,
        span: Span,
    }

    #[derive(Debug, Ast)]
    pub struct Binary {
        #[cst(find = "cst::Calc::Value")]
        left: Box<Value>,
        #[cst(find = "NodeId::TOKEN")]
        op: BinOp,
        #[cst(find = "cst::Calc::Value")]
        right: Box<Value>,
        span: Span,
    }

    #[derive(Debug, Ast)]
    pub struct BinOp {
        span: Span,
    }

    #[derive(Debug, Ast)]
    pub struct UnOp {
        span: Span,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use test_case::test_case;

        #[test_case(r#"5"#, "number")]
        #[test_case(r#"1+2"#, "infix")]
        #[test_case(r#"1+2 * 5"#, "preced")]
        #[test_case(r#"(1 + 2)"#, "parent")]
        #[test_case(r#"-5"#, "unary")]
        #[test_case(r#"-(1 + 2)"#, "unary_parent")]
        fn expr_test(input: &str, test_case_name: &str) {
            let expr = Value::from_str(input).unwrap();
            let actual_debug = format!("```\n{}\n```\n{:#?}", input, expr);

            alder::testing::snap(actual_debug, file!(), test_case_name);
        }
    }
}
/*
mod ast {


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
*/

fn main() {}
