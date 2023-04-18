// Add to use WebSocket::split
use futures::StreamExt;
use warp::hyper::{Response, StatusCode};
use std::fs;
use std::{env, net::SocketAddr};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
// use map function 
use warp::{Filter, Reply, Rejection};

static NEXT_USERID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

#[tokio::main]
async fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    println!("Address: {}", addr);

    let socket_address: SocketAddr = addr.parse().expect("Cannot find a valid socket address");

    let users = Users::default();
    let users_a = users.clone();
    let users_b = users.clone();
    let users = warp::any().map(move || users_a.clone());
    let users_conn = warp::any().map(move || users_b.clone());

    let hello_opt = warp::path::param::<String>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<String>,), std::convert::Infallible>((None,)) });

    // PATH: /hello/xxxx
    let hello_endpoint = warp::path("hello")
        .and(hello_opt)
        .and(warp::path::end())
        .map(|name: Option<String>| {
            format!("Hello, {}!", name.unwrap_or_else(|| "world".to_string()))
        });

    let ws_endpoint = warp::path("ws")
        .and(warp::ws())
        .and(users)
        .map(| ws: warp::ws::Ws, users | ws.on_upgrade(move | socket | connect(socket, users)));

    let ws_connected_endpoint = warp::path("connected")
        .and(users_conn)
        .and_then(connected_users);

    let files = warp::fs::dir("./static");

    let res_404 = warp::any().map(|| {
        warp::http::Response::builder()
            .status(warp::http::StatusCode::NOT_FOUND)
            .body(fs::read_to_string("./static/404.html").expect("404 404?"))
    });

    let routes = ws_endpoint
        .or(hello_endpoint)
        .or(ws_connected_endpoint)
        .or(files)
        .or(res_404);

    let server = warp::serve(routes).try_bind(socket_address);

    println!("Running server at {}!", addr);

    server.await
}

async fn connected_users(users: Users) -> std::result::Result<impl Reply, Rejection> {
    println!("Request connected users...");
    let size = users.read().await.len();
    Ok(format!("Connected users #{}", size))
}


async fn connect(ws: WebSocket, users: Users) {
    let new_id = NEXT_USERID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    println!("User with ID: {} connected!", new_id);
    
    // Split the ws transmit and recieve streams
    let (user_tx, mut user_rx) = ws.split();
    // Create a tunnel to talk with all the sockets
    let (tx, rx) = mpsc::unbounded_channel();

    let rx = UnboundedReceiverStream::new(rx);

    tokio::spawn(rx.forward(user_tx));
    users.write().await.insert(new_id, tx);

    // Reading and broadcasting messages
    while let Some(result) = user_rx.next().await {
        println!("Broadcast message from {}", new_id);
        broadcast_msg(result.expect("Failed to fetch message"), &users, new_id).await;
    }

    disconnect(new_id, &users).await;
}

async fn broadcast_msg(msg: Message, users: &Users, user_id: usize) {
    if let Ok(_) = msg.to_str() {
        for (&_uid, tx) in users.read().await.iter() {
            let formatted_message = format!("{}: {:}", user_id, msg.to_str().unwrap());
            tx.send(Ok(Message::text(formatted_message))).expect("Failed to send message");
        }
    }
}

async fn disconnect(user_id: usize, users: &Users) {
    println!("User with ID {} disconnected", user_id);
    users.write().await.remove(&user_id);
}