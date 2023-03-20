use std::sync::mpsc::channel;

use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

use mini_redis::{client, server};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>
    }
}

#[tokio::main]
async fn main() {
    // Create a channel to send messages between tasks. Set buffer up to the provided number of messages, the capacity of the channel
    let (tx, mut rx) = mpsc::channel(32);
    
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let serv_resp = client.get(&key).await;
                    let _ = resp.send(serv_resp);
                }
                Command::Set { key, val, resp } => {
                    let serv_resp = client.set(&key, val).await;
                    let _ = resp.send(serv_resp);
                }
            }
        }
    });
    
    let tx2 = tx.clone();

    let task_a = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get { 
            key: "krusty".to_string(),
            resp: resp_tx
        };
        tx.send(cmd).await.unwrap();
        let resp = resp_rx.await;
        println!("GOT = {:?}", resp.unwrap());
    });

    let task_b = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set { 
            key: "krusty".to_string(), 
            val: "Rusty".into(),//Bytes::from("Rusty"),
            resp: resp_tx
        };
        tx2.send(cmd).await.unwrap();

        let resp = resp_rx.await;
        println!("GOT: {:?}", resp);
    });

    // WARNING: Ensure that the tasks (join handles) are fully complete
    // If we do not do that there is a risk that our messages
    // does not reach the server because the execution STOPS
    task_a.await.unwrap();
    task_b.await.unwrap();

    manager.await.unwrap();
}