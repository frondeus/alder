use alder::lexer::*;
use alder::parser::*;
use alder::*;

node_kinds! {
    Json:
        Whitespace,

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
    v_node(|s| s
        .parse(ws())
        .peek(|c, s| match c {
            None => todo!("value"),
            Some(c) => match c {
                't' | 'f' => s.parse(boolean()),
                '[' => s.parse(array()),
                '{' => s.parse(object()),
                '-' | '.' | '0'..='9' => s.parse(number()),
                '"' => s.parse(string()),
                //c if c.is_whitespace() => s.parse(ws()),
                c => {
                    dbg!(&s);
                    todo!("{:?}", c)
                },
            },
        })
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
    v_node(|mut s| {
        let mut end = false;
        loop {
            s = s
                .parse(value())
                .parse(ws())
                .peek(|c, s| match c {
                None => todo!("inner_array"),
                Some(']') => {
                    end = true;
                    s.skip()
                }
                _ => s.parse(','),
            });
            if end {
                return s;
            }
        }
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
