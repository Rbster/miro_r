use axum::{extract::State, response::IntoResponse, routing::get, Router};
use axum_macros::debug_handler;
use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};
use serde::{Deserialize, Serialize};
use std::{f64, time::Instant};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let state = MyState { users: vec![] };
    // Make a regular axum router
    let app = Router::new().route("/ws", get(handler)).with_state(state);

    // Run it!
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

#[debug_handler]
async fn handler(
    // Upgrade the request to a WebSocket connection where the server sends
    // messages of type `ServerMsg` and the clients sends `ClientMsg`
    ws: WebSocketUpgrade<ServerMsg, ClientMsg>,
    State(state): State<MyState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| ping_pong_socket(socket, state))
}

// Send a ping and measure how long time it takes to get a pong back
async fn ping_pong_socket(mut socket: WebSocket<ServerMsg, ClientMsg>, myState: MyState) {
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
    Coord(Coord),
}

#[derive(Debug, Deserialize)]
enum ClientMsg {
    Pong,
    Coord(Coord),
}

#[derive(Debug, Clone)]
struct MyState {
    users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Uuid,
    coord: Coord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Coord {
    x: f64,
    y: f64,
}
