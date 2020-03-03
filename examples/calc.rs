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
            Binary,
            Unary,
            Number
    }

    use derive_more::Display;
    #[derive(Debug, Display, Clone)]
    enum Problem {
        #[display(fmt = "Expected either digit or one of `+`, `-`, `/`, `*`, `(`")]
        UnexpectedToken,

        #[display(fmt = "Expected infix operator")]
        UnexpectedInfix,
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
    /// 2 ^ 3
    /// (2 ^ 3)
    /// (2 ^^^ 3)
    #[alder_test]
    pub fn value() -> impl Parser {
        with_extra(
            extra(),
            pratt(Calc::Binary,
                  vec![Calc::Value],
                  |state: &mut State| left_value().parse(state),
                  |state| match state.peek(1).as_ref() {
                      "*" => Some((20, token("*").boxed())),
                      "/" => Some((20, token("/").boxed())),
                      "+" => Some((10, token("+").boxed())),
                      "-" => Some((10, token("-").boxed())),
                      "" | ")" => None,
                      _ => Some((100, raise(Problem::UnexpectedInfix, 1).boxed())),
            })
        )
    }

    fn left_value() -> impl Parser {
        v_node(Calc::Value, |state| {
            match state.peek(1).as_ref() {
                s if s.is_digits() => state.add(number()),
                "(" => {
                    state.add("(".as_extra());
                    state.add(value());
                    state.add(")".as_extra());
                },
                "-" => {
                    state.add(minus());
                },
                _ => state.add(raise(Problem::UnexpectedToken, 1)),
            };
        })
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
    use crate::cst::Calc;
    use crate::*;
    use alder::{CstIterExt, FromCst, Node, NodeId, Span, State};

    #[derive(Debug)]
    pub enum Value {
        Number(Number),
        Unary(Unary),
        Binary(Binary),
        Error(Node),
    }

    impl FromCst for Value {
        fn from_node(node: &Node) -> Option<Self> {
            if !node.is(Calc::Value) {
                return None;
            }
            Number::from_node(node)
                .map(Self::Number)
                .or_else(|| Unary::from_node(node).map(Self::Unary))
                .or_else(|| Binary::from_node(node).map(Self::Binary))
                .or_else(|| Some(Self::Error(node.clone())))
        }
    }

    impl Value {
        pub fn eval(&self) -> Option<i32> {
            match self {
                Self::Number(n) => n.eval(),
                Self::Unary(n) => n.eval(),
                Self::Binary(n) => n.eval(),
                Self::Error(_) => None,
            }
        }
    }

    #[derive(Debug)]
    pub struct Number {
        value: i32,
        span: Span,
    }

    impl FromCst for Number {
        fn from_node(node: &Node) -> Option<Self> {
            if !node.is(Calc::Number) {
                return None;
            }
            Some(Self {
                value: node.span.as_ref().parse().unwrap(),
                span: node.span.clone(),
            })
        }
    }

    impl Number {
        pub fn eval(&self) -> Option<i32> {
            Some(self.value)
        }
    }

    #[derive(Debug)]
    pub struct Unary {
        op: UnOp,
        right: Box<Value>,
        span: Span,
    }

    impl FromCst for Unary {
        fn from_node(node: &Node) -> Option<Self> {
            if !node.is(Calc::Unary) {
                return None;
            }
            let mut iter = node.children.iter();
            let op = iter.find_cst()?;
            let right = iter.find_cst().map(Box::new)?;
            Some(Self {
                op,
                right,
                span: node.span.clone(),
            })
        }
    }

    impl Unary {
        pub fn eval(&self) -> Option<i32> {
            let right = self.right.eval()?;
            match self.op {
                UnOp::Min => Some(-right),
            }
        }
    }

    #[derive(Debug)]
    pub struct Binary {
        left: Box<Value>,
        op: BinOp,
        right: Box<Value>,
        span: Span,
    }

    impl FromCst for Binary {
        fn from_node(node: &Node) -> Option<Self> {
            if !node.is(Calc::Binary) {
                return None;
            }
            let mut iter = node.children.iter();
            let left = iter.find_cst().map(Box::new)?;
            let op = iter.find_cst()?;
            let right = iter.find_cst().map(Box::new)?;
            Some(Self {
                left,
                op,
                right,
                span: node.span.clone(),
            })
        }
    }

    impl Binary {
        pub fn eval(&self) -> Option<i32> {
            let left = self.left.eval()?;
            let right = self.right.eval()?;
            Some(match self.op {
                BinOp::Add => left + right,
                BinOp::Sub => left - right,
                BinOp::Div => left / right,
                BinOp::Mul => left * right,
            })
        }
    }

    #[derive(Debug)]
    pub enum BinOp {
        Add,
        Mul,
        Sub,
        Div,
    }

    impl FromCst for BinOp {
        fn from_node(node: &Node) -> Option<Self> {
            if !node.is(NodeId::TOKEN) {
                return None;
            }
            match node.span.as_ref() {
                "+" => Some(Self::Add),
                "*" => Some(Self::Mul),
                "-" => Some(Self::Sub),
                "/" => Some(Self::Div),
                _ => None,
            }
        }
    }

    #[derive(Debug)]
    pub enum UnOp {
        Min,
    }

    impl FromCst for UnOp {
        fn from_node(node: &Node) -> Option<Self> {
            if !node.is(NodeId::TOKEN) {
                return None;
            }
            match node.span.as_ref() {
                "-" => Some(Self::Min),
                _ => None,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use test_case::test_case;

        #[test_case(r#"5"#, "number")]
        #[test_case(r#"1+2"#, "infix")]
        #[test_case(r#"1+2*5"#, "preced")]
        #[test_case(r#"(1 + 2)"#, "parent")]
        #[test_case(r#"-5"#, "unary")]
        #[test_case(r#"-(1 + 2)"#, "unary_parent")]
        #[test_case(r#"-(1 + err)"#, "error")]
        #[test_case(r#"1 + 1 ^ 2"#, "error2")]
        fn expr_test(input: &str, test_case_name: &str) {
            let cst = State::parse(input, crate::cst::value());
            let mut nodes = cst.nodes.iter();
            let val = nodes.find_cst::<Value>().unwrap();
            let actual = val.eval();

            let actual_debug = format!("{}\n{:#?}\n{:?}", &cst, val, actual);
            alder::testing::snap(actual_debug, file!(), test_case_name);
        }
    }
}

fn main() {
    #[cfg(feature = "derive")]
    {
        use crate::ast::Value;
        use alder::{CstIterExt, State};

        let arg = std::env::args().nth(1);
        match arg {
            Some(input) => {
                print!("{}", &input);
                let cst = State::parse(&input, crate::cst::value());
                let mut nodes = cst.nodes.iter();
                let val = nodes.find_cst::<Value>().unwrap();
                if let Some(actual) = val.eval() {
                    println!("= {}", actual);
                } else {
                    println!();
                    println!("{}", &cst);
                }
            }
            None => eprintln!("Expected at least one arg."),
        }
    }
}
