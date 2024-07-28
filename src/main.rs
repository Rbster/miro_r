use axum::{response::IntoResponse, routing::get, Router};
use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};
use serde::{Deserialize, Serialize};
use std::{f64, time::Instant};

#[tokio::main]
async fn main() {
    // Make a regular axum router
    let app = Router::new().route("/ws", get(handler));

    // Run it!
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

async fn handler(
    // Upgrade the request to a WebSocket connection where the server sends
    // messages of type `ServerMsg` and the clients sends `ClientMsg`
    ws: WebSocketUpgrade<ServerMsg, ClientMsg>,
) -> impl IntoResponse {
    ws.on_upgrade(ping_pong_socket)
}

// Send a ping and measure how long time it takes to get a pong back
async fn ping_pong_socket(mut socket: WebSocket<ServerMsg, ClientMsg>) {
    let mut start = Instant::now();
    socket.send(Message::Item(ServerMsg::Ping)).await.ok();

    println!("started");

    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Item(ClientMsg::Pong)) => {
                println!("ping: {:?}", start.elapsed());
                start = Instant::now();
                socket.send(Message::Item(ServerMsg::Ping)).await.ok();
            }
            Ok(_) => {}
            Err(err) => {
                eprintln!("got error: {}", err);
                break;
            }
        }
    }
}

#[derive(Debug, Serialize)]
enum ServerMsg {
    Ping,
}

#[derive(Debug, Deserialize)]
enum ClientMsg {
    Pong,
}

#[derive(Debug)]
struct State {
    users: Vec<User>,
}

#[derive(Debug)]
struct User {
    id: u64,
    x: f64,
    y: f64,
}
