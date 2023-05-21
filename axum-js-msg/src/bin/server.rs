use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};

use axum_js_msg::{MyRequest, MyResponse};
use futures::{sink::SinkExt, stream::StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/websocket", get(websocket_upgrade));

    println!("Starting server on 'ws://127.0.0.1:3000'");
    axum::Server::bind(&"127.0.0.1:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn websocket_upgrade(ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("Got a connection, upgrading to a WebSocket.");
    ws.on_upgrade(|socket| websocket_handler(socket))
}

async fn websocket_handler(stream: WebSocket) {
    let (mut ws_tx, mut ws_rx) = stream.split();

    println!("Listening for messages on the WebSocket.");
    while let Some(message) = ws_rx.next().await {
        match message {
            Ok(message) => match message {
                Message::Text(message) => {
                    let message: MyRequest = serde_json::from_str(&message).unwrap();
                    println!("Got request with name: '{}'.", message.name);
                    let response = MyResponse {
                        message: format!("Hello, {}!", message.name),
                    };
                    ws_tx
                        .send(Message::Text(serde_json::to_string(&response).unwrap()))
                        .await
                        .unwrap();
                }
                Message::Close(_) => {
                    println!("Got a close message.");
                    break;
                }
                m => println!("Got an unimplemented message: {:?}", m),
            },
            Err(e) => eprintln!("Error reading message from WebSocket: {}", e),
        }
    }
    println!("Handler exiting.");
}
