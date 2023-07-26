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
            let data = stream.write(&message.as_bytes());
            match data {
                Ok(_) => println!("Server #1, has sent {:?} messages", i + 1),
                Err(e) => {
                    println!("ERR on #1: {:?}", e.to_string());
                    break;
                }
            }
        }
    }
    if label == 2 {
        for i in 0..20 {
            let random_sleep_time = rand::thread_rng().gen_range(1500..2500);
            thread::sleep(Duration::from_millis(random_sleep_time));
            let message = format!("Awake after {}ms sleeping", random_sleep_time);
            let data = stream.write(&message.as_bytes());
            match data {
                Ok(_) => println!("Server #2, has sent {:?} messages", i + 1),
                Err(e) => {
                    println!("ERR on #2: {:?}", e.to_string());
                    break;
                }
            }
        }
    }
    if label == 3 {
        for _ in 0..20 {
            let random_sleep_time = rand::thread_rng().gen_range(2000..4000);
            thread::sleep(Duration::from_millis(random_sleep_time));
            let message = format!("Awake after {}ms sleeping", random_sleep_time);
            // Write in the stream three times in a row
            stream.write(&message.as_bytes()).expect("#3 stream is broken");
            // let data = stream.write(&message.as_bytes());
            // match data {
            //     Ok(_) => println!("Server #3, has sent {:?} messages", i + 1),
            //     Err(e) => {
            //         println!("ERR on #3: {:?}", e.to_string());
            //     }
            // }
            // let data2 = stream.write(&message.as_bytes());
            // match data2 {
            //     Ok(_) => println!("Server #3, has sent {:?} messages", i + 1),
            //     Err(e) => {
            //         println!("ERR on #3: {:?}", e.to_string());
            //     }
            // }
            // let data3 = stream.write(&message.as_bytes());
            // match data3 {
            //     Ok(_) => println!("Server #3, has sent {:?} messages", i + 1),
            //     Err(e) => {
            //         println!("ERR on #3: {:?}", e.to_string());
            //     }
            // }
        }
        println!("Finished");
    }
}

fn main()  -> std::io::Result<()>{
    let server_one = TcpListener::bind("127.0.0.1:8000").expect("8000 PORT busy");
    let server_two = TcpListener::bind("127.0.0.1:8001").expect("8001 PORT busy");
    let server_three = TcpListener::bind("127.0.0.1:8002").expect("8002 PORT busy");

    println!("Waiting for listeners...");

    thread::spawn(move|| {
        for client_stream in server_one.incoming() {
            println!("In #1 server incoming request...");
            let stream = client_stream.unwrap();
            handle_client(stream, 1);
        }
    });
    
    thread::spawn(move|| {
        for client_stream in server_two.incoming() {
            println!("In #2 server incoming request...");
            let stream = client_stream.unwrap();
            handle_client(stream, 2);
        }
    });
    
    thread::spawn(move|| {
        for client_stream in server_three.incoming() {
            println!("In #3 server incoming request...");
            let stream = client_stream.unwrap();
            handle_client(stream, 3);
        }
    });
    loop {}
}