use alder::lexer::*;
use alder::parser::*;
use alder::*;

node_kinds! {
    Json:
        // Extra
        Whitespace,
        InlineComment,
        OutlineComment,

        // Nodes
        Value,
        String,
        InnerString,
        Array,
        Object,
        Number,
        Bool
}

#[alder]
fn ws() -> impl Parser {
    chomp_while(Json::Whitespace, |c| c.is_whitespace())
}

fn is_newline_or_eof(c: char) -> bool {
    c == '\n' || c == '\r'
}

fn is_not_newline_or_eof(c: char) -> bool { !is_newline_or_eof(c) }

/// //foo
/**
    //foo bar baz
    not foo
*/
/// /* Outline comment */
/// /* Outline comment */ not a comment
/**
    /* Multiline
        comment
        */ not a comment
*/
#[alder]
fn comment() -> impl Parser {
    v_node(|s| s.peek(|c, s| match c {
        Some('/') => s.peek_nth(1, |c, s| match c {
            Some('/') => s.parse(chomp_while(Json::InlineComment, is_not_newline_or_eof)),
            Some('*') => s.parse(out_comment()),
            _ => s.skip()
        }),
        _ => s.skip()
    }))
}

#[alder]
fn out_comment() -> impl Parser {
    node(Json::OutlineComment, |s| {
        s.parse(repeat(|s, end| {
            s.peek(|c, s| match c {
                Some('*') => s.peek_nth(1, |c, s| match c {
                    Some('/') => {
                        *end = true;
                        s.consume(2)
                    },
                    _ => s.consume(1)
                }),
                _ => s.consume(1)
            })
        }))
    })
}

/// //foo
/**
    //comment
    // also comment


                not a comment
*/
#[alder]
fn extras() -> impl Parser {
    v_node(|s| s.parse(
        repeat(|s, end| {
            s.peek(|c, s| match c {
                Some(c) if c.is_whitespace() => s.parse(ws()),
                Some('/') => s.parse(comment()),
                _ => {
                    *end = true;
                    s.skip()
                }
            })
        }))
    )
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
/**
    // comment
    5.0 // comment 2
*/
#[alder]
fn value() -> impl Parser {
    v_node(|s| s
        .parse(extras())
        .peek(|c, s| match c {
            None => todo!("value"),
            Some(c) => match c {
                't' | 'f' => s.parse(boolean()),
                '[' => s.parse(array()),
                '{' => s.parse(object()),
                '-' | '.' | '0'..='9' => s.parse(number()),
                '"' => s.parse(string()),
                c => {
                    dbg!(&s);
                    todo!("{:?}", c)
                },
            },
        })
        .parse(extras())
    )
}

/// true
/// false
#[alder]
fn boolean() -> impl Parser {
    v_node(|s| s
        .peek(|c, s| match c {
            Some('t') => s.parse(tag(Json::Bool, "true")),
            Some('f') => s.parse(tag(Json::Bool, "false")),
            _ => todo!("bool"),
        })
    )
}

/// []
/// [true]
/// [true,false]
/// [true, false ]
/// [ true, false ]
/// [ true, false]
/// [ true , false ]
#[alder]
fn array() -> impl Parser {
    node(Json::Array, |s| s
        .parse('[')
        .peek(|c, s| match c {
            None => todo!("array"),
            Some(']') => s.skip(),
            _ => s.parse(inner_array())
        })
        .parse(']')
    )
}

#[alder]
fn inner_array() -> impl Parser {
    v_node(|s| {
        s.parse(repeat(|s, end| s
            .parse(value())
            .parse(extras())
            .peek(|c, s| match c {
                None => todo!("inner_array"),
                Some(']') => {
                    *end = true;
                    s.skip()
                }
                _ => s.parse(','),
            })
        ))
    })
}

/// {}
#[alder]
fn object() -> impl Parser {
    node(Json::Object, |s| s
        .parse('{')
        .parse('}')
    )
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
    node(Json::String, |s| s
        .parse('"')
        .peek(|c, s| match c {
            None => todo!("string"),
            Some('"') => s,
            _ => s.parse(chomp_while(Json::InnerString, |c| c != '"')),
        })
        .parse('"')
    )
}
