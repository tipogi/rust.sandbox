use std::io::prelude::*;
use std::str::FromStr;
use std::{net::{TcpListener, TcpStream}, io::Read};

const KARIN_SERVER_ADDRESS: &str = "127.0.0.1:8001";
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    println!("karin starting...");
    // Start listening
    let listener = TcpListener::bind(KARIN_SERVER_ADDRESS).unwrap();
    // Start
    println!("karin listening {}", KARIN_SERVER_ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection openned!!");
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    // read the buffer
    let mut buffer = [0; 1024];
    let length = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..length]);
    println!("received: {}", message);
    let sirocco_message = call_sirocco(message.to_owned().to_string());
    let output = format!("sirocco says: {}", sirocco_message);
    // write the message
    let _ = stream.write_all(output.as_bytes());
    println!("message sent: {}", output);
}

fn call_sirocco(message: String) -> String{
    // Conection message
    println!("connecting to sirocco: {}...", SIROCCO_SERVER_ADDRESS);
    // Establish connection with remote TCP server host
    // With that connect, the thread is blocked because it is an synchronous connection
    // It calls a blocking call because it waits until it returns the socket and nothing
    // more can execute in that thread
    if let Ok(mut stream) = TcpStream::connect(SIROCCO_SERVER_ADDRESS) {
        // Because socat is doing a fork in each connection, the port is different
        // if we compare with echo_server_address
        println!("Connected to sirocco server!{}:{}", 
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );
        // => write in the socket
        // => Create the message in buffer, in that case bytes
        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();
        println!("sent to sirocco: {}", message);
        // => Read the result
        // Create a buffer to write when the stream receive data
        // The length might be as we want. In that case, 100, 100bytes, cannot read more
        let mut buffer = [0;100];
        let len = stream.read(&mut buffer).unwrap();
        // Is gonna handle not valid characters also with lossy
        let message = String::from_utf8_lossy(&buffer);
        println!("received from sirocco: {} with {} length", message, len);
        return message.to_owned().to_string();
    } else {
        println!("Failed to connect to sirocco server {}", SIROCCO_SERVER_ADDRESS);
        return String::from_str("failed to connect to sirocco").unwrap();
    }
}