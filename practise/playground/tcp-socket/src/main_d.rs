use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};

const TCP_SERVER:&str = "localhost:8080";
const CHANNEL_PARTICIPANTS: usize = 10;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(TCP_SERVER).await.unwrap();
    // Change the turbofish or delete. Automatically will interpret the compiler
    let (tx, _rx) = broadcast::channel(CHANNEL_PARTICIPANTS);
    // Wait for each client
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (read, mut writter) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();
            loop {
                // We will use tokio macro select to divide the two task that in the previous
                // example it was blocking the program.
                // This will let as to run multiple asynchronous things concurrently at the same time
                // and act on the first one that comes back with a result.
                // We could create two spawns to achieve the same things but this is more elegant
                tokio::select! {
                    // Give a future and then a code block
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            println!("The client closed connection");
                            break;
                        }
                        // Push in the broadcast channel (internal) the incoming message
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        // The message that we receive from the broadcast channel (internal)
                        let (msg, sender_addr) = result.unwrap();
                        if sender_addr != addr {
                            // Send back to the connection that the client open
                            writter.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }                
            }
        });
    }

}
