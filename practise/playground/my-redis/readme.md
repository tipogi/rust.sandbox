# Rust mini-redis

## Run example file

The folder _name_ has to be in plural (examples) and the _command_ in singular

```rust
cargo run --example hello-redis
```

## Create bin folder

Move server and client files to bin folder

```bash
mkdir src/bin
mv src/main.rs src/bin/server.rs
```

and create a new binary file that will contain the client code:

```bash
touch src/bin/client.rs
# Run the server 
cargo run --bin server
# Run the client
cargo run --bin client
```
