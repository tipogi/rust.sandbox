## Keywords

- macro
- Panic
- Pattern matching
- heap
- ownership
- views or slices, references

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

## Videos

- [Introduction to Rust Part 1](https://www.youtube.com/watch?v=WnWGO-tLtLA)
- [The Rust Lang Book](https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8)
- [Pointers and dynamic memory - stack vs heap](https://www.youtube.com/watch?v=_8-ht2AKyH4)
- [Visualizing memory layout of Rust's data types](https://www.youtube.com/watch?v=rDoqT-a6UFg)

## Sources

- [Learn Rust](https://www.rust-lang.org/learn): Rust official documentation
- [Docs rs](https://docs.rs/): Docs.rs is an open source documentation host for crates
- [Rust Playground](https://play.rust-lang.org/)
- [Rust Language Cheat Sheet](https://cheats.rs/)
- [Ideomatic Rust](https://github.com/grunch/idiomatic-rust): A peer-reviewed collection of articles/talks/repos which teach concise, idiomatic Rust.
- [Rustlings](https://github.com/rust-lang/rustlings): Small exercises to get you used to reading and writing Rust code!
- [Crates.io](https://crates.io/): The Rust community’s crate registry
- [Take your first steps with Rust](https://learn.microsoft.com/en-us/training/paths/rust-first-steps/)

## Books

- Programming Rust: Fast, Safe Systems Development by Jason Orendorff, Jim Blandy, and Leonora F.S. Tindall
- The Rust Programming Language by Steve Klabnik and Carol Nichols
