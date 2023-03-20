# Echo Play

Created the client and server to play around with synchronous and asynchronous requests.
For that, we use `std::io` and `tokio`

The echo-play returns what we type, it is kind of mirror.

Tokio is a key rust framework that is used as an asynchronous runtime that powers many web frameworks such as axum, actix-web or rocket.rs.
By understanding tokio, this will give you a deeper understanding of those frameworks and how they're built (and how you use them) as well as understanding async await when performing any i/o bound programming.

## Exercise: Clone that use case with Rust

Install the following packages:

- __netcat__: We will use as an a echo client
- __socat__: TCP echo server

- `socat -v tcp-l:1234, fork exec:'/bin/cat'`: Create a tcp listener (tcp-l) on port 1234.
When someone hit the server, spin off new connection connection and call `cat` command
- `nc 127.0.0.1 1234`: Our client. Send some word to tcp server. Once we are connect, type words

## Source

[Chris Hay](https://www.youtube.com/watch?v=DJzgUmH30h8)
