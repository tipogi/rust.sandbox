use std::{net::SocketAddr, io::{self, Write}};

use clap::Parser;
use mio::net::TcpStream;
use tlsclient::{args::Args, client_config, tls_client::TlsClient};

fn lookup_ipv4(host: &str, port: u16)  -> SocketAddr {
    use std::net::ToSocketAddrs;

    let addrs = (host, port).to_socket_addrs().unwrap();
    for addr in addrs {
        if let SocketAddr::V4(_) = addr {
            return addr
        }
    }
    unreachable!("Cannot lookup address")
}


fn main() {
    
   let args = Args::parse();

   println!("[TLS_Client, mains.rs] CLI args:\n {:#?}", args);

    if args.verbose {
        env_logger::Builder::new()
            .parse_filters("trace")
            .init();
    }
    
    let addr = lookup_ipv4(args.host.as_str(), args.port);
    
    let config = client_config::make_config(&args);

    let sock = TcpStream::connect(addr).unwrap();

    let server_name = args
        .host
        .as_str()
        .try_into()
        .expect("[TLS_Client, mains.rs] ERROR: Invalid DNS name");

    let mut tsl_client = TlsClient::new(sock, server_name, config);

    if args.http {
        println!("[TLS_Client, mains.rs] Make an HTTP request to the server...");
        let httpreq = format!(
            "GET / HTTP/1.0\r\nHost: {}\r\nConnection: \
                               close\r\nAccept-Encoding: identity\r\n\r\n",
            args.host
        );
        tsl_client
            .write_all(httpreq.as_bytes())
            .unwrap();
    } else {
        println!("[TLS_Client, mains.rs] Read source");
        let mut stdin = io::stdin();
        tsl_client
            .read_source_to_end(&mut stdin)
            .unwrap();
    }

    println!("[TLS_Client, mains.rs] Create a new poll");
    let mut poll = mio::Poll::new().unwrap();
    
    println!("[TLS_Client, mains.rs] Set events limit");
    let mut events = mio::Events::with_capacity(32);

    println!("[TLS_Client, mains.rs] Register the poll in the TLS client");
    tsl_client.register(poll.registry());

    loop {
        poll.poll(&mut events, None).unwrap();

        for ev in events.iter() {
            tsl_client.ready(ev);
            tsl_client.reregister(poll.registry());
        }
    }
}