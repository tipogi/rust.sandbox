use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}};

const TCP_SERVER:&str = "localhost:8080";

#[tokio::main]
async fn main() {
    // Suspend the current task until it resolves the future
    let listener = TcpListener::bind(TCP_SERVER).await.unwrap();
    // Wait for each client
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        // A task is a UNIT OF WORK in the async world, as a lightweight thread. They are not OS
        // threads not OS tasks.
        // They are units of asynchronous computation that tokio knows about and is able to very
        // efficiently interleave with each other and schedule in an optimal ways.
        // It knows how to sleep a task until its I/O resource is ready and then, as soon as the I/O
        // resource is ready, wake up the task and continue processing.
        // So to get some concurrency in the program, to handle multiple clients in the same time
        // we need tokio::spawn to create a new task
        // In that case, we are going to wrap all the client interaction. This is going to move
        // all of the clients handling onto its own independent task
        tokio::spawn(async move {
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
                // Mirror to the client, the message that the client sends
                writter.write_all(line.as_bytes()).await.unwrap();
                // If not it stores all the messages
                line.clear();
            }
        });
    }

}
