use std::io::prelude::*;
use std::{net::{TcpListener, TcpStream}, io::Read};
use std::env::args;
use std::{thread, time::Duration};

const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    // Read arguments
    let delay = args().nth(1).unwrap_or_default().parse::<u64>().unwrap();
    println!("Delay ENV set in {}ms", delay);
    println!("sirocco starting...");
    // Start listening
    let listener = TcpListener::bind(SIROCCO_SERVER_ADDRESS).unwrap();
    // Start
    println!("sirocco listening {}", SIROCCO_SERVER_ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection openned!!");
        handle_connection(stream, delay)
    }
}

fn handle_connection(mut stream: TcpStream, delay: u64) {
    println!("handling the connection...");
    // read the buffer
    let mut buffer = [0; 1024];
    let length = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..length]);
    println!("received: {}", message);
    // Delay the message
    println!("sleeping {}ms...", delay);
    thread::sleep(Duration::from_millis(delay));
    // write the message
    let _ = stream.write_all(message.as_bytes());
    println!("message sent: {}", message);
}
