# Create project

```bash
cargo new [project-name]
```

`.toml` is the manifest file of the project

Like in all binary files, the main function is `main` function

## Keywords

- macro
- Panic
- Pattern matching
- heap
- ownership
- views or slices, references

## Commands

- typecheck: `cargo check`. It does not create the binary
- Create the binari: First, it does type check after, build the binary file: `cargo build`
- Release binary: `cargo build --release`
- Run the binary: It builds the binary and after runs the binary. `cargo run`

## File system

- target: All the build artifact are there, including, end binary
  - debug: we will find our end binary. In that case `kvstore`
  - release: Create a release binary
- Cargo.lock: It explains to cargo the lock versions of the dependencies

## Environment

- env: usually heavier files because it has debug files
- release: lighter because the compiler does not do as  many optimisation

## What is

- systems programing language which meant for low-level software, OS, web-browsers, foundational software... Sometimes is also good option to build high-level software but is focused in low-level.
- Sometimes might be difficult because we have to care about low-level details.
- Statically typed programing language. Rust compiler will infer all of the types of the args, binding, ...
- It is not object oriented program language, there is nothing like classes. In rust usually are `struct`s

## Tooling

- clippy
- rust-analyzer

## Interesting

- [functions vs macro](https://youtu.be/WnWGO-tLtLA?t=3147)
- Difference about stack vs heap
- [Borrowing and ownership](https://youtu.be/WnWGO-tLtLA?t=5988)
- [Views](https://youtu.be/WnWGO-tLtLA?t=6359)

## Test

```bash
cargo run -- hello world
## With cargo-watch (first we need to install)
cargo watch -x 'run -- hello world' 
```

## Code

Power of `?`: when we need to handle error control after expression

```rust
let contents = match std::fs::read_to_string("kv.db") {
    // equal to Result::Ok but because Result is in the scope we can write Ok
    Ok(c) => c,
    Err(error) => {
        return Result::Err(error);
        // because in rust Result is common type we could return like that 
        //return Err(error)
    }
}
// It is equivalent to above expression: (NOTE sometime it does not work with Result type)
let contents = std::fs::read_to_string("kv.db")?;
```

## Source

- [Introduction to Rust Part 1](https://www.youtube.com/watch?v=WnWGO-tLtLA)
