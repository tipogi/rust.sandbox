# Rust Sandbox

<p align="center"><img src="./images/rust-cover.png" alt="Rust cover"></p>

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
- To check the trace of the error: `RUST_BACKTRACE=1 cargo run`
- Test library with output: `cargo test -- --show-output`

## Visual Studio Code Plugins

- `rust-analyzer`: Rust language server

## File system

- target: All the build artifact are there, including, end binary
  - debug: we will find our end binary. In the case of ryan-levick/kvstore, the binary will be `kvstore`
  - release: Create a release binary
- Cargo.lock: It explains to cargo the lock versions of the dependencies

## Environment

- env: usually heavier files because it has debug files
- release: lighter because the compiler does not do as  many optimisation

## Ownership Rules

- Each value in Rust has a variable that's called its owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

## References

- At any given time, we can have either one mutable reference or any number of immutable references
- References must always be valid

## Sources

- [Cargo Book](https://doc.rust-lang.org/cargo/index.html): Cargo is the Rust package manager
- [Learn Rust](https://www.rust-lang.org/learn): Rust official documentation
- [The Rust reference](https://doc.rust-lang.org/reference/introduction.html): This book does not serve as an introduction to the language. Background familiarity with the language is assumed
- [Crates.io](https://crates.io/): The Rust community’s crate registry
- [Rust by example](https://doc.rust-lang.org/rust-by-example/index.html): Rust by Example (RBE) is a collection of runnable examples that illustrate various Rust concepts and standard libraries
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Rust Standard library](https://doc.rust-lang.org/std/index.html): API reference
- [Rust by Practise](https://practice.rs/why-exercise.html): This book was designed for easily diving into and get skilled with Rust, and it's very easy to use: All you need to do is to make each exercise compile without ERRORS and Panics !
- [Docs rs](https://docs.rs/): Docs.rs is an open source documentation host for crates
- [Rust Playground](https://play.rust-lang.org/)
- [Rust Language Cheat Sheet](https://cheats.rs/): A single-page Rust resource for people who like high information density.
- [Ideomatic Rust](https://github.com/grunch/idiomatic-rust): A peer-reviewed collection of articles/talks/repos which teach concise, idiomatic Rust.
- [Rustlings](https://github.com/rust-lang/rustlings): Small exercises to get you used to reading and writing Rust code!
- [Take your first steps with Rust](https://learn.microsoft.com/en-us/training/paths/rust-first-steps/)
- [Curso de Rust para desarrolladores JavaScript](https://midu.dev/rust-para-desarrolladores-javascript/)

## Videos

- Ryan Levick's, [Introduction to Rust](https://www.youtube.com/watch?v=WnWGO-tLtLA)
- Let's get Rusty's, [The Rust Lang Book](https://www.youtube.com/watch?v=OX9HJsJUDxA&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8)
- Rust Tutorial, [Doug Milford](https://www.youtube.com/watch?v=2KTG3OQPPJ4&list=PLLqEtX6ql2EyPAZ1M2_C0GgVd4A-_L4_5&index=3)
- Derek Banas, [Rust tutorial full course](https://www.youtube.com/watch?v=ygL_xcavzQ4)
- [Pointers and dynamic memory - stack vs heap](https://www.youtube.com/watch?v=_8-ht2AKyH4)
- [Visualizing memory layout of Rust's data types](https://www.youtube.com/watch?v=rDoqT-a6UFg)
- [Jeff No Zhao](https://www.youtube.com/playlist?list=PLkO5ggdQuRaaeFke7nWS4ajhFVZ1biE7_)

## Channels

- [Jon Gjengset](https://www.youtube.com/@jonhoo/videos)
- [Ryan Levicks](https://www.youtube.com/@RyanLevicksVideos/videos)
- [Let's get Rusty](https://www.youtube.com/@letsgetrusty/videos)
- [Faster than lime](https://www.youtube.com/@fasterthanlime)
- [Jacques](https://www.youtube.com/@jacques-dev)

## Databases

- [SurrealDB](https://surrealdb.com/): It is the ultimate cloud database for tomorrow's applications

## Books

- `Programming Rust`: Fast, Safe Systems Development by Jason Orendorff, Jim Blandy, and Leonora F.S. Tindall
- `The Rust Programming Language` by Steve Klabnik and Carol Nichols
