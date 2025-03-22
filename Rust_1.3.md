# __Chapter 1.3 : Hello, Cargo__

Cargo = Rust's build system and package manager

.TOML = 'Tom's Obvious, Minimal Language' format

## Steps to run Cargo Project

`cargo build` = Creates executable in target/debug/hello_cargo.exe

`./target/debug/hello_cargo` = Runs the file

`cargo run` = Faster way that combines Step 1 and 2

## Checking Rust code

`cargo check` = Checks code to make sure it compiles without executable

- This is faster than `cargo build`

## Building for Release

`cargo build --release` = Compiles it with optimizations

Executable to __target/release__ instead of __target/debug__

- Makes Rust code run faster
- Makes compiling longer
