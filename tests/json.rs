use alder::{lexer::*, parser::*, *};

node_kinds! {
    Json:
        // Tokens
        OCurly,
        CCurly,
        DoubleQuote,
        OSquare,
        CSquare,
        Whitespace,

        // Nodes
        Value,
        String,
        InnerString,
        Array,
        Object,
        Number
}

#[alder]
fn whitespace() -> impl Parser {
    chomp_while(Json::Whitespace, |c| c.is_whitespace())
}

/// "foo"
/// "foo",more
/// 5.0
/**
    {
        "foo": "bar,
        "baz": 2
    }
*/
/// [ 4.0, 5.0, "foo" ]
#[alder]
fn value() -> impl Parser {
    node(Json::Value,
         peek_char(|c| match c {
             '"' => string().boxed(),
             '[' => array().boxed(),
             '{' => object().boxed(),
             '-' | '.' | '0'..='9' => number().boxed(),
             c => todo!("{:?}", c)
         })
    )
}

/// []
/// [5.0]
#[alder]
fn array() -> impl Parser {
    node(Json::Array, (
        token(Json::OSquare, '['),
        peek_char_2(|c| match c {
            ']' => token(Json::CSquare, ']').boxed(),
            _ => (
                value(),
                token(Json::CSquare, ']')
            ).boxed()
        })
    ))
}

/// {}
#[alder]
fn object() -> impl Parser {
    node(Json::Object, (
        token(Json::OCurly, '{'),
        token(Json::CCurly, '}'),
    ))
}

/// 5.0
/// 5
/// -1
/// -1.0
/// 0
/// 0,more
/// error
/// .10
#[alder]
fn number() -> impl Parser {
    /*
    Maybe its not an elegant solution but... works :)
    */
    let json_number = r"^[-]?((0|[1-9][0-9]*)(\.[0-9]+)?|\.[0-9]+)([eE][+-]?[0-9]+)?";
    let r = regex::Regex::new(json_number).unwrap();

    crate::lexer::regex(Json::Number, r)
}

/// "foo"
/// "foo",more
/**
    {
        "foo": "bar,
        "baz": 2
    }
*/
/// ""
#[alder]
fn string() -> impl Parser {
    node(
        Json::String,
        (
            token(Json::DoubleQuote, '"'),
            chomp_while(Json::InnerString, |c| c != '"'),
            token(Json::DoubleQuote, '"'),
        ),
    )
}
