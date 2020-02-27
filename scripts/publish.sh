#!/usr/bin/env bash

cargo fmt --all
cargo clippy --all-features --all-targets --all -- -D warnings || exit

#nvim derive/Cargo.toml
nvim Cargo.toml
nvim README.md

git add .
git commit

#cd derive/ || exit
#cargo publish --dry-run || exit

#cargo publish

#sleep 3

#cd ../
cargo publish --dry-run || exit

sleep 3

cargo publish
