[![Crates.io](https://img.shields.io/crates/v/alder.svg)](https://crates.io/crates/alder)
[![Docs.rs](https://docs.rs/alder/badge.svg)](https://docs.rs/alder)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rust-lang/docs.rs/master/LICENSE)
![Build](https://github.com/frondeus/alder/workflows/Build/badge.svg)

# Alder

### Warning: Library is under development and may be subject to change.

Hand written recursive descent and non-backtracking parsing "combinator" library designed with nice error in mind
and lossless data.

## Install
Use cargo-edit:
```sh
cargo add alder
```

Or add it manually:
```toml
alder =  "0.2.0"
```

You may want to enable a derive feature as well:

```toml
alder = { version = "0.2.0" , features = ["derive"] }
```

## Example
```rust
// Doc tests are treated as a test cases for snapshots.
// To enable them use `derive` feature and add `#[alder_test]` macro.
// It also supports /** multiline comments */
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
/**
    [
        true,
        false,
        "foo"
    ]
*/
#[alder_test]
fn array() -> impl Parser {
    // with_extra injects whitespace parser before and after every token.
    // Unless you explicitly told parser not to do it (for example in strings).
    with_extra(ws(), node(Json::Array, |state| {
        state.add(token('['));
        match state.input.peek() {
            Some(']') => (),
            _ => 'outer: loop {
                state.add(value());
                'inner: loop { // Until we find either ']' or ','
                    match state.input.peek() {
                        Some(']') => {
                            break 'outer;
                        }
                        Some(',') => {
                            // If there was a problem and we find `,` we try to process rest of the array normally.
                            state.add(recover(token(',')));

                            // Trailing comma
                            if let Some(']') = state.input.peek() { 
                                break 'outer;
                            }
                            break 'inner;
                        },
                        // EOF
                        None => { 
                            state.add(raise(Problem::InvalidTokenArray, 1));
                            break 'outer;
                        },
                        _ => state.add(raise(Problem::InvalidTokenArray, 1)),
                    };
                }
            },
        }
        state.add(recover(token(']')));
    }))
}
```

Parsers should return information about what happened and where it happened:
```
--------------------------------- SYNTAX ERROR ---------------------------------
I was parsing Boolean when found issue:
 0 |[truadsadsa, falsa]\EOF
 ~ | ^^^^^^^^^^ I expected `true`

--------------------------------- SYNTAX ERROR ---------------------------------
I was parsing Boolean when found issue:
 0 |[truadsadsa, falsa]\EOF
 ~ |             ^^^^^ I expected `false`
```
