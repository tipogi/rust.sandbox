use std::{sync::{Arc, Mutex}, collections::HashMap};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::{net::{TcpListener, TcpStream}};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:6379";
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Mini redis listening on {:?}", address);

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        println!("mini-redis waiting conncetions...");
        let (socket, from) = listener.accept().await.unwrap();
        println!("external connection from: {:?}", from);

        // Create a clone of database
        let db_clone = db.clone();

        // A new TASK is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        // Like this we add concurrency to the application and we do not need to 
        // await block the thread until process is completed
        tokio::spawn(async move {
            let hello = "Tokio task created";
            println!("{:?}", hello);
            process(socket, db_clone).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by 'mini-redis' handles frames from
    // the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("External connection wants to write in the HashMap: SET");
                // Wait until we get the control of the lock
                let mut db = db.lock().unwrap();
                db.insert(
                    cmd.key().to_string(),
                    cmd.value().clone()
                );
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("External connection wants to red a key from the HashMap: GET");
                // Wait until we get the control of the lock
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}