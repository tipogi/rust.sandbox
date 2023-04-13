# GRPC-DEMO

Create a client and server to communicate through gRPC protocol

## Install Protoc

Install in the linux machine, protocol buffer compiler `protoc` to generate data access
classes in rust from the `.proto` definition

```bash
apt install -y protobuf-compiler
```

## Proto folder

Define the gRPC service and messages which are the types needed for the calls.

## build.rs

Add a build tooling that will hook the `cargo build` step to compile the `.proto` file during every build.

When it runs `cargo build` it will create a `src/store.rs` file automatically

## v2

- Replace the hashmap from mysql database for data persistency
- Create a database:
  - Install globally sqlx-cli: `cargo install sqlx-cli`
  - **Create new database**: When we spin up the server, if it does not have the server a database, it will create new one:
    - `let arc_dbm = Arc::new(Mutex::new(dbm));` -> main.rs
  - Before create a migration, check if the db has the desired content
    - Install sqlite3: `sudo apt-get install sqlite3`
    - `sqlite3 store.db`, enter in the prompt of db
    - `.tables`, show the db tables
  - Once it has the content the db, create migration: `sqlx migrate add store` and it will export the database in migrations folder

## Sources

- [gRPC docs](https://grpc.io/docs/)
- [Protocol Buffers](https://protobuf.dev/overview/)
