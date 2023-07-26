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
    println!("The socket listening on localhost:8080...");
    // Wait for each client
    loop {
        //Accept a connection from the client
        let (mut socket, _addr) = listener.accept().await.unwrap();
        println!("=> Connected {} client", _addr.to_string());
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
            println!("Waiting to get some input from the client...");
            let bytes_read = reader.read_line(&mut line).await.unwrap();
            println!("readed bytes: {}", bytes_read);
            println!("input Line: {}", line);
            // The reader has reach the end of file, there is not more data left
            // or the clients exit
            if bytes_read == 0 {
                println!("<== bye bye {} client", _addr);
                break;
            }
            writter.write_all(line.as_bytes()).await.unwrap();
            // If not it stores all the messages
            line.clear();
        }
    }

}
