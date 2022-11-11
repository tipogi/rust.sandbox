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

## Source

- [Introduction to Rust Part 1](https://www.youtube.com/watch?v=WnWGO-tLtLA)
