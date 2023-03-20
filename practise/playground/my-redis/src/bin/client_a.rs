use std::time::Duration;

use bytes::Bytes;
use tokio::sync::mpsc;

use mini_redis::client;

enum Command {
    Get {
        key: String
    },
    Set {
        key: String,
        val: Bytes
    }
}

#[tokio::main]
async fn main() {
    // Establish a connection to the server
    //let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // Create a channel to send messages between tasks. Set buffer up to the provided number of messages, the capacity of the channel
    let (tx, mut rx) = mpsc::channel(32);

    let tx2 = tx.clone();


    let task_a = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(3000)).await;
        tx.send("Hello from task 1, after sleep 3000ms").await;
    });

    let task_b = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1200)).await;
        tx2.send("Hello from task 2, after sleep 1200ms").await;
    });

    /*while let Some(message) = rx.recv().await {
        println!("Message received: {:?}", message);
    }*/

    // IMPORTANT
    // If we add inside the task the reciver instead of the while loop (above) 
    // and we do not await the task, is going to finish earlier that the task
    // are completed
    let manager = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("Message received: {:?}", message);
        }
    });

    task_a.await.unwrap();
    task_b.await.unwrap();

    manager.await.unwrap();
}