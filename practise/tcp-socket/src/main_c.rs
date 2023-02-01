use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};

const TCP_SERVER:&str = "localhost:8080";
const CHANNEL_PARTICIPANTS: usize = 10;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(TCP_SERVER).await.unwrap();
    // Create a broadcast channel with multiple producers and multiple receivers
    // that has that functionality in one channel
    // In every task, we will have a receiver and sender
    // In the way that we define the type, it calls turbofish
    // Turbofish is an operator that is pretty unique to rust, it is a way to hint
    // the compiler, what kind of generic type we expect to be returned from a function
    // IMPORTANT: This is not a socket, it is an internal communication layer, queue
           
    // Wait for each client
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        // We need to clone because we are moving the tx into the new task (tokio::spawn) within a loop
        let tx = tx.clone();
        // Instead of clone as we did with sender, in that case we can clone subcribe
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (read, mut writter) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();
            // Wait that the client will disconnect
            loop {
                // PROBLEM: Because we are blocking until we receive a message,
                // the task is stuck because it cannot receive and send nothing.
                // The one that is going to receive is going to be client that 
                // type the message. All other clients task in the server will 
                // be awaiting. When they will type, they will get the last
                // word that they did not consume.
                println!("Wating for messages...");
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                println!("Received line: {}", bytes_read);
                // After read the line, check if it is some message
                if bytes_read == 0 {
                    println!("The client closed connection");
                    break;
                }
                // It add in the queue of the broadcast channel the message
                tx.send(line.clone()).unwrap();
                // The local channel consumes the last message in the queue
                let msg = rx.recv().await.unwrap();
                // And send back
                writter.write_all(msg.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }

}
