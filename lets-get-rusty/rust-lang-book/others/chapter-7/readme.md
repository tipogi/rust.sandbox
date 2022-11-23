# Chapter 7 of rust-lang-book

As the project grows, we need a way to organise and encapsulate the code  

- `module system`: Starts with creation of package when we run `cargo new [project_name]` and packages, store crates.
- `crate`: Could be a binary that we could execute or library crate which is the code that could used in different programs. Crates contain modules.
- `module`: Modules allow us to organise a chunk of code and control the privacy rules
- `workspaces`: It is used in large projects and allows to store interelated packages inside the workspace

## Crate

Packages store __crates__ and we can define different crates inside of `Cargo.toml` file. By default a package has a binary crate which is compiled from the `src` folder. Rust follows one convention that if we have `/src/main.rs` defined in the source directory, then the binary crate with the same name as the package would be automatically created and `main.rs` would be the __crate root__. The __crate root__ is a source file that the rust compiler starts up when building the __crate__. It also makes up root module of the __crate__. There is similar convention for library __crates__.

When we create `lib.rs` file in the root of the `src` directory, rust will automatically create a __library crate__ with the same name as the package and `lib.rs` would be the root of the library.

Crate has some rules:

- A package must have at least one crate
- A package could have either zero library crate or one library crate
- A package could have any number of binary crates
- If we want more `binary crates`, we would create a `bin` directory under `src`

## Modules

Create a new package with `library crate`

```rust
cargo new --lib restaurant
```

Modules are specified with `mod` keyword followed by the name of the module. Modules can contain other modules inside of them. They could also contain struct, enum, constants, traits and so on.

At the top of module tree, we have module called _crate_. Crate is the module created by default for crate root (in our case `lib.rs`). A good analogy is to think about module tree like the directory tree.

The rust privacy rules by default, child modules and everything inside of it, is private from the prespective of parent module. On the flip side, child modules are able to see anything that is defined in the parent module. This system allows to hide implementation details by default and only expose the functions we need to the outside world. If we want to expose, add `pub` keyword.