# Rust Sandbox

<p align="center"><img src="./rust-cover.png" alt="Rust cover"></p>

In this repository, you can find some steps to start learning Rust languague. Mostly, it uses youtube platform playlist to get the knowledge but also can be some useful portals

## What is Rust?

- systems programing language which meant for low-level software, OS, web-browsers, foundational software... Sometimes is also good option to build high-level software but is focused in low-level.
- Sometimes might be difficult because we have to care about low-level details.
- Statically typed programing language. Rust compiler will infer all of the types of the args, binding, ...
- It is not object oriented program language, there is nothing like classes. In rust usually are `struct`s

## Create project

```bash
cargo new [project-name]
```

`.toml` is the manifest file of the project

Like in all binary files, the main function is `main` function

## Commands

- typecheck: `cargo check`. It does not create the binary
- Create the binari: First, it does type check after, build the binary file: `cargo build`
- Release binary: `cargo build --release`
- Run the binary: It builds the binary and after runs the binary. `cargo run`

## Visual Studio Code Plugins

- `rust-analyzer`: Rust language server

## File system

- target: All the build artifact are there, including, end binary
  - debug: we will find our end binary. In that case `kvstore`
  - release: Create a release binary
- Cargo.lock: It explains to cargo the lock versions of the dependencies

## Environment

- env: usually heavier files because it has debug files
- release: lighter because the compiler does not do as  many optimisation

## Youtube

- Ryan Levick's, [Introduction to Rust](https://www.youtube.com/watch?v=WnWGO-tLtLA)
- Let's get Rusty's, [The Rust Lang Book](https://www.youtube.com/watch?v=OX9HJsJUDxA&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8)
