use std::{net::{self}};
use clap::Parser;
use mio::net::{TcpListener};

use tlsserver::{args::Args, server_mode::ServerMode, tls_server::{TlsServer, LISTENER}, server_config::make_config};

fn main() {
   let args = Args::parse();
   println!("CLI args:\n {:#?}", args);

   if args.verbose {
    env_logger::Builder::new()
        .parse_filters("trace")
        .init();
    }

   let addr: net::SocketAddr = format!("{}:{}", args.host, args.port).parse().unwrap();
   // Configure the TLS server
   let config = make_config(&args);

    // Create a TCP listener to listen the connections
   let mut listener = TcpListener::bind(addr).expect("ERROR: Cannot listen on that port");
   let mut poll = mio::Poll::new().unwrap();

   poll.registry()
    .register(&mut listener, LISTENER, mio::Interest::READABLE)
    .unwrap();

    let mode = match args.mode.as_str() {
        "forward"   => ServerMode::Forward(9000),
        "echo"      => ServerMode::Echo,
        _           => ServerMode::Http
    };

    let mut tls_server = TlsServer::new(listener, mode, config);

    let mut events = mio::Events::with_capacity(256);

    loop {
        println!("Waiting for poll events...");
        poll.poll(&mut events, None).unwrap();
        println!("Received new events in the poll, iterate the events");
        let mut event_number = 0;
        for event in events.iter() {
            event_number += 1;
            println!("EVENT number: {:?} and TOKEN name {:?}", event_number, event.token());
            match event.token() {
                LISTENER => {
                    println!("LISTENER: New incoming EVENT, accept connection");
                    tls_server
                        .accept(poll.registry())
                        .expect("ERROR: Error accepting socket")
                }
                _ => tls_server.conn_event(poll.registry(), event),
            }
        }
    }
}