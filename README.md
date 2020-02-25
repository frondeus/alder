[![Crates.io](https://img.shields.io/crates/v/alder.svg)](https://crates.io/crates/alder)
[![Docs.rs](https://docs.rs/alder/badge.svg)](https://docs.rs/alder)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rust-lang/docs.rs/master/LICENSE)
![Build](https://github.com/frondeus/alder/workflows/Build/badge.svg)

# Alder

### Warning: Library is under development and may be subject to change.

Hand written recursive descent and non-backtracking parsing "combinator" library designed with nice error in mind
and lossless data.

### Goals
* [x] Almost no backtracking
* [x] Lossless tree generation (for formatters and IDE approach)
* [x] UTF-8 support
* [ ] Nice informative errors with contexts.
* [ ] AST generation based on CST (some macro and trait magic)

I'm somehow inspired by [this post](https://matklad.github.io/2018/06/06/modern-parser-generator.html). It's about parser generators but I prefer writing them manually.

### TODO
* [ ] Documentation (right now I have only WIP JSON example)
* [ ] Maybe incremental parsing...

## Install
Use cargo-edit:
```sh
cargo add alder
```

Or add it manually:
```toml
alder =  "0.5.0"
```

You may want to enable a derive feature as well:

```toml
alder = { version = "0.5.0" , features = ["derive"] }
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
