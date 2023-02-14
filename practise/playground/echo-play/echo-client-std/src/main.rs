// Import all the libraries from std::io to not write all the path
use std::io::prelude::*;
// We are going to connect to the soca echo server via tcp connection
// So we will need tcp stream class
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

pub fn main() {
    // Conection message
    println!("connecting to {}...", ECHO_SERVER_ADDRESS);
    // Establish connection with remote TCP server host
    // With that connect, the thread is blocked because it is an synchronous connection
    // It calls a blocking call because it waits until it returns the socket and nothing
    // more can execute in that thread
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        // Because socat is doing a fork in each connection, the port is different
        // if we compare with echo_server_address
        println!("Connected to echo server!{}:{}", 
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );
        // => write in the socket
        let message = "Hello Rust from SYNC std! Lets code";
        // => Create the message in buffer, in that case bytes
        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();
        println!("send: {}", message);
        // => Read the result
        // Create a buffer to write when the stream receive data
        // The length might be as we want. In that case, 100, 100bytes, cannot read more
        let mut buffer = [0;100];
        let len = stream.read(&mut buffer).unwrap();
        // Is gonna handle not valid characters also with lossy
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {} with {} length", message, len);
    } else {
        println!("Failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
        println!("Run socat with following command: socat -v tcp-l:1234, fork exec:'/bin/cat'")
    }
}
