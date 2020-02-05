use alder::{lexer::*, parser::*, *};

node_kinds! {
    Json:
        DoubleQuote,
        String,
        InnerString
}

/// "foo"
/// "foo",more
/**
    {
        "foo": "bar,
        "baz": 2
    }
*/
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
