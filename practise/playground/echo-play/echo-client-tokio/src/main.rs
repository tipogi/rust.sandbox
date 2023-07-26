use tokio::io::{AsyncWriteExt, AsyncReadExt};
// We are going to connect to the soca echo server via tcp connection
// So we will need tcp stream class
use tokio::net::TcpStream;

const SIROCCO_SERVER_ADDRESS: &str = "localhost:8000";
const KARIN_SERVER_ADDRESS: &str = "localhost:8001";

// Add tokio macro. In that case, it does not make sense to add asynchronous process
// because we just have single thread app
#[tokio::main]
async fn main() {
    // Conection message
    println!("connecting to {}...", KARIN_SERVER_ADDRESS);
    // Establish connection with remote TCP server host: Asynchronous, non-blocking model
    // With that model, it calls to await and that thread essentially would
    // be returned back to the pool
    // In that case, it can process other things on that thread and execute
    //
    // Rust does not have an asynchronous runtime as NodeJS and because of that
    // we need a thread pool underneath of that, it needs a worker poll to manage
    // those io type connections.
    //
    // Thats what provides an asynchronous runtime, a thread pool and for that we have tokio
    // Futures: Abstractions for asynchronous
    if let Ok(mut stream) = TcpStream::connect(KARIN_SERVER_ADDRESS).await {
        // Because socat is doing a fork in each connection, the port is different
        // if we compare with echo_server_address
        println!("Connected to echo server!{}:{}", 
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );
        // => write in the socket
        let message = "TCP tokio client";
        // => Create the message in buffer, in that case bytes
        let _ = stream.write_all(message.as_bytes()).await;
        println!("send: {}", message);
        // => Read the result
        // Create a buffer to write when the stream receive data
        // The length might be as we want. In that case, 100, 100bytes, cannot read more
        let mut buffer = [0;100];
        let len = stream.read(&mut buffer).await.unwrap();
        // Is gonna handle not valid characters also with lossy
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {} with {} length", message, len);
    } else {
        println!("Failed to connect to echo server {}", KARIN_SERVER_ADDRESS);
        println!("Run socat with following command: socat -v tcp-l:1234, fork exec:'/bin/cat'")
    }
}
