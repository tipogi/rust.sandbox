use std::{thread, time::Duration, io, future::Future};
use asynchronous::futurama::{ FuturamaInfo, FuturamaApiEndpoints, FuturamaCast };
use tokio::net::TcpListener;

const TCP_SERVER:&str = "localhost:8080";
const LIMIT_REQ: usize = 20;
const SLEEP_MS: usize = 1500;

#[tokio::main]
async fn main() {
    let mut info_counter: usize = 0;
    let mut cast_counter: usize = 0;
    let listener = TcpListener::bind(TCP_SERVER).await.unwrap();
    let mut loop_counter = 0;
    loop {
        // Enters in a race, who is going to finish first the task
        // I guess, the task that not win the race, would be cancelled
        // I AM NOT SURE ABOUT. Still learning
        // Tokio doc: https://tokio.rs/tokio/tutorial/select
        // Now the race happen between the first two branches. If we set false,
        // load_await_req of three and four branches the race will enter between then
        // because it process faster that others
        println!("### Race start between branches ###");
        tokio::select! {
            futurama_info = reqwest::get(FuturamaApiEndpoints::Info.url()) => {
                if info_counter < LIMIT_REQ {
                    let _json_response = futurama_info
                        .unwrap()
                        .json::<Vec<FuturamaInfo>>()
                        .await
                        .unwrap();
                    //println!("Futurama: {:?}", json_response[0].get_creators());
                    info_counter += 1;
                    println!("\t#API: Futurama INFO fetched {} times!", info_counter);
                } else {
                    break;
                }
            }
            futurama_cast = reqwest::get(FuturamaApiEndpoints::Cast.url()) => {
                if cast_counter < LIMIT_REQ {
                    let _json_response = futurama_cast
                    .unwrap()
                    .json::<Vec<FuturamaCast>>()
                    .await
                    .unwrap();
                    //println!("Futurama characters: {:?}", json_response);
                    cast_counter += 1;
                    println!("\t#API: Futurama CHARACTERS {} times fetched!", cast_counter);
                } else {
                    break;
                }
            }
            greeting = instantaneous_greeting(true) => {
                println!("\t#GREETINGS from {}", greeting.unwrap());
            }
            _ = listener.accept() => {
                // cmd: telnet localhost 8080
                println!("\tconnection opened from #TELNET, in!");
            }
            _cli_input = wait_for_user_input(true) => {
                //println!("{:?}", cli_input.);
                println!("\tuser #INPUT {:?}","Executed");
            }
        }
        loop_counter += 1;
        println!("{}.==> End loop! sleep {} ms <==", loop_counter, SLEEP_MS);
        thread::sleep(Duration::from_millis(SLEEP_MS as u64));
    }
}

// Transformed Future implemtation or we can say syntactic sugar. When we define before fn
// keyword async, it is implicit the future definition. This and the above one functions,
// are equivalent if we refer to function signature
// ALERT: That std::io might be dangerous => https://youtu.be/ThjvMReOXYM?t=3329
async fn wait_for_user_input(load_await_req: bool) -> Option<String> {
    if load_await_req {
        let _one = reqwest::get(FuturamaApiEndpoints::Cast.url()).await;
        println!("@one-input: Request finished");
        let _one = reqwest::get(FuturamaApiEndpoints::Cast.url()).await;
        println!("@two-input: Request finished");
    }
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading line");
    
    let input = buffer.parse::<String>().to_owned().unwrap();
    Some(input)
}

// Manual async definition. Because the function it is not defined as async but the function body
// it is asynchronous, we define the return type as future
// In the body we also need to define that it is asynchronous
// Note: In JS we name to the Futures, promises
fn instantaneous_greeting(load_await_req: bool) -> impl Future<Output=Option<String>> {
    async move {
        if load_await_req {
            let _one = reqwest::get(FuturamaApiEndpoints::Cast.url()).await;
            println!("@one-inst: Request finished");
            let _one = reqwest::get(FuturamaApiEndpoints::Cast.url()).await;
            println!("@two-inst: Request finished");
        }
        Some(String::from("Rusty Krusty!"))
    }
}
