# AoC 2021 Solutions in Rust

Goals:
- Become more familiar with stdlib
- Error handling
- Generics, traits
- Automated testing
- Workspaces, package structure

## Usage
`cargo build && cargo run`

Each day has its own binary and there is one main binary which will run all puzzles.
Currently, just using `cargo run` is not sufficient, because it doesn't pick up changes
in the bin directory.