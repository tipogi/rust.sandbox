use std::{net::{TcpListener, TcpStream}, thread, time::Duration, io::{Write}};

use rand::Rng;

fn handle_client(mut stream: TcpStream, label: u32) {
    println!("Label: {:?}", label);
    if label == 1 {
        println!("Server #1, is going to write 20 messages...");
        for i in 0..20 {
            let random_sleep_time = rand::thread_rng().gen_range(1000..2000);
            thread::sleep(Duration::from_millis(random_sleep_time));
            let message = format!("Awake after {}ms sleeping", random_sleep_time);
            //stream.write(&message.as_bytes()).unwrap();
            let data = stream.write(&message.as_bytes());
            match data {
                Ok(_) => println!("Server #1, has sent {:?} messages", i + 1),
                Err(e) => {
                    println!("ERR on #1: {:?}", e.to_string());
                    break;
                }
            }
            stream.write("Timer Post-message".as_bytes()).unwrap();
        }
    }
    println!("Finished");
}

fn main()  -> std::io::Result<()>{
    let server_one = TcpListener::bind("127.0.0.1:8000").expect("8000 PORT busy");

    println!("Waiting for listeners...");

    thread::spawn(move|| {
        for client_stream in server_one.incoming() {
            println!("In #1 server incoming request...");
            let stream = client_stream.unwrap();
            handle_client(stream, 1);
        }
    });

    loop {}
}