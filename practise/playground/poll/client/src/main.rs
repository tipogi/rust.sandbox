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
        // Register the streams in the poll
        poll.registry().register(&mut client_one, mio::Token(1), mio::Interest::READABLE).unwrap();

        Self {
            clients: vec![client_one],
            poll,
            events,
            client_total: vec![0]
        }
    }

    fn poll(&mut self) {
        // Wait for readiness events. It will store the incoming events in events property
        self.poll.poll(&mut self.events, None).unwrap();
        let mut att = 0;
        println!("{:?}", &self.events);
        for event in &self.events {
            att += 1;
            if event.token() == mio::Token(1) && event.is_readable() {
                let mut buffer = [0; 1024];
                //let length = self.clients[0].read(&mut buffer).unwrap();
                //let message = String::from_utf8_lossy(&buffer[..length]);
                let length = self.clients[0].read(&mut buffer).expect("Something went wrong with the #1 stream");
                let message = str::from_utf8(&buffer[..length]).unwrap();
                println!("Remote peer message: {:?}", message);
                self.client_total[0] += 1;
            }
        }
        println!("Current Poll attempt total: {}", att);
    }
}

fn main() {
    let mut executor = Executor::new();

    loop {
        //println!("Waiting to some incoming events...");
        if executor.client_total[0] == 20 {
            break;
        }
        executor.poll()
    }
}
