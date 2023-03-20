use std::io::Read;
use std::{str, vec};
use mio::net::TcpStream;

use mio::Events;

struct Executor {
    clients: Vec<TcpStream>,
    poll: mio::Poll,
    events: mio::Events,
    client_total: Vec<u32>
}

impl Executor {
    fn new() -> Self {

        // Create a mio Poll to receive the events
        let poll = mio::Poll::new().unwrap();
        // Limit the readiness events of the poll
        let events = Events::with_capacity(30);
        // Create the stream to listen some response
        let mut client_one = TcpStream::connect("127.0.0.1:8000".parse().unwrap()).expect("The server is not listening on PORT 8000");
        let mut client_two = TcpStream::connect("127.0.0.1:8001".parse().unwrap()).expect("The server is not listening on PORT 8001");
        let mut client_three = TcpStream::connect("127.0.0.1:8002".parse().unwrap()).expect("The server is not listening on PORT 8002");
        // Register the streams in the poll
        poll.registry().register(&mut client_one, mio::Token(1), mio::Interest::READABLE).unwrap();
        poll.registry().register(&mut client_two, mio::Token(2), mio::Interest::READABLE).unwrap();
        poll.registry().register(&mut client_three, mio::Token(3), mio::Interest::READABLE).unwrap();

        Self {
            clients: vec![client_one, client_two, client_three],
            poll,
            events,
            client_total: vec![0,0,0]
        }
    }

    fn poll(&mut self) {
        // Wait for readiness events. It will store the incoming events in events property
        self.poll.poll(&mut self.events, None).unwrap();
        let mut att = 0;
        for event in &self.events {
            //println!("{:?}", event);
            att += 1;
            if event.token() == mio::Token(1) && event.is_readable() {
                let mut buf = [0;10];
                match self.clients[0].read_exact(&mut buf) {
                    Err(e) => println!("{:?}", e),
                    Ok(func) => func 
                }
                self.client_total[0] += 1;
            } else if event.token() == mio::Token(2) && event.is_readable() {
                let mut buf = [0;10];
                self.clients[1].read_exact(&mut buf).expect("The server is not listening on PORT 8001");
                self.client_total[1] += 1;
            } else if event.token() == mio::Token(3) && event.is_readable() {
                let mut buf = [0;20];
                self.clients[2].read_exact(&mut buf).unwrap();
                let h_string = str::from_utf8(&buf).unwrap();
                println!("msg: {:?}, {:?}", h_string, buf);
                self.client_total[2] += 1;
                println!("Stream #3 read {} times", self.client_total[2]);
            }
        }
        println!("Current Poll attempt total: {}", att);
    }
}

fn main() {
    let mut executor = Executor::new();

    loop {
        //println!("Waiting to some incoming events...");
        if executor.client_total[0] == 20 && executor.client_total[1] == 20 && executor.client_total[2] == 20 {
            break;
        }
        executor.poll()
    }
}
