use std::str::FromStr;
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};
use uuid::Uuid;

const KARIN_SERVER_ADDRESS: &str = "127.0.0.1:8001";
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    println!("karin starting...");
    // Start listening
    let listener = TcpListener::bind(KARIN_SERVER_ADDRESS).await.unwrap();
    // Start
    println!("karin listening {}", KARIN_SERVER_ADDRESS);

    loop {
        // Tokio accept, instead of incomming implementation of sync
        let (stream, _) = listener.accept().await.unwrap();
        println!(" => connection openned!!");
        //handle_connection(stream).await;
        // Instead of using a single thread, as we do above and block the current thread,
        // we create multiple threads to manage the request an we can process concurrently.
        // So, when we use spawn, each thread is going to open a new tcp connection
        // and it is not going to block all the runtime
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    // read the buffer
    let mut buffer = [0; 1024];
    let length = stream.read(&mut buffer).await.unwrap();
    let message = String::from_utf8_lossy(&buffer[..length]);
    println!("received: {}", message);
    let sirocco_message = call_sirocco(message.to_owned().to_string()).await;
    let output = format!("sirocco says: {}", sirocco_message);
    // write the message
    let _ = stream.write_all(output.as_bytes()).await;
    println!("DONE: closed the connection!");
}

async fn call_sirocco(message: String) -> String{
    let id = Uuid::new_v4().to_simple();
    // Conection message
    println!("connecting to sirocco: {}...", SIROCCO_SERVER_ADDRESS);
    // Establish connection with remote TCP server host
    // With that connect, the thread is blocked because it is an synchronous connection
    // It calls a blocking call because it waits until it returns the socket and nothing
    // more can execute in that thread
    if let Ok(mut stream) = TcpStream::connect(SIROCCO_SERVER_ADDRESS).await {
        // Because socat is doing a fork in each connection, the port is different
        // if we compare with echo_server_address
        println!("=> Connected to sirocco server!{}:{}", 
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );
        // => write in the socket
        // => Create the message in buffer, in that case bytes
        let _ = stream.write_all(message.as_bytes()).await;
        println!("==> {}: sent to sirocco...", id);
        // => Read the result
        // Create a buffer to write when the stream receive data
        // The length might be as we want. In that case, 100, 100bytes, cannot read more
        let mut buffer = [0;100];
        let len = stream.read(&mut buffer).await.unwrap();
        println!("Message length: {}", len);
        // Is gonna handle not valid characters also with lossy
        let message = String::from_utf8_lossy(&buffer);
        println!("Message: {}", message);
        println!("<== {}: received from sirocco", id);
        return message.to_owned().to_string();
    } else {
        println!("Failed to connect to sirocco server {}", SIROCCO_SERVER_ADDRESS);
        return String::from_str("failed to connect to sirocco").unwrap();
    }
}