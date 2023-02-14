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
                tokio::select! {
                    // Give a future and then a code block
                    // -> Branch one
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            println!("The client closed connection");
                            break;
                        }
                        // Push in the broadcast channel (internal) the incoming message
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    // ->    Branch two
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
