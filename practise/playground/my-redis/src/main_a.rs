use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:6379";
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Mini redis listening on {:?}", address);

    loop {
        println!("mini-redis waiting conncetions...");
        let (socket, from) = listener.accept().await.unwrap();
        println!("external connection from: {:?}", from);
        // A new TASK is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        // Like this we add concurrency to the application and we do not need to 
        // await block the thread until process is completed
        tokio::spawn(async move {
            let hello = "Tokio task created";
            println!("{:?}", hello);
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut db = HashMap::new();

    // Connection, provided by 'mini-redis' handles frames from
    // the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("External connection wants to write in the HashMap: SET");
                db.insert(
                    cmd.key().to_string(),
                    cmd.value().to_vec()
                );
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("External connection wants to red a key from the HashMap: GET");
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