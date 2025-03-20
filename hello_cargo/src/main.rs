// Chapter 1.3: Hello, Cargo!

// Cargo = Rust's build system and package manager
// .TOML = 'Tom's Obvious, Minimal Language' format

fn main()
{
    println!("Hello, world!");
}

// Steps to run Cargo Project
// 1. 'cargo build' = Creates executable in target/debug/hello_cargo.exe
// 2. './target/debug/hello_cargo' = Runs the file
// OR
// 3. 'cargo run' = Combines Step 1 and 2

// 'cargo check' = Checks code to make sure it compiles without executable
// This is faster than 'cargo build'