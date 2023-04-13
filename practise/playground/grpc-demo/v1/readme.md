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

## Sources

- [gRPC docs](https://grpc.io/docs/)
- [Protocol Buffers](https://protobuf.dev/overview/)
