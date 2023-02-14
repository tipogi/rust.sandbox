use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}};

const TCP_SERVER:&str = "localhost:8080";

// Even we are using here tokio, all the code works synchronously
// We are blocking the same thread all over
// In other words, we have a single task and that just runs through in 
// a linear fashion 
#[tokio::main]
async fn main() {
    // Suspend the current task until it resolves the future
    let listener = TcpListener::bind(TCP_SERVER).await.unwrap();
    // Wait for each client
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        // Split the write and read part of the socket because the reader is going to take
        // the ownership of the socket
        let (read, mut writter) = socket.split();
        // It wraps any kind of reader and it maintains its own buffer
        // Allows you to do some higher level read operations as read line
        // of a text stream
        let mut reader = BufReader::new(read);
        let mut line = String::new();
        // Wait that the client will disconnect
        loop {
            let bytes_read = reader.read_line(&mut line).await.unwrap();
            // The reader has reach the end of file, there is not more data left
            // or the clients exit
            if bytes_read == 0 {
                break;
            }
            writter.write_all(line.as_bytes()).await.unwrap();
            // If not it stores all the messages
            line.clear();
        }
    }

}
