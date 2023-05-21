use websockets::{Frame, WebSocket};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ws = WebSocket::connect("ws://127.0.0.1:3000/websocket").await?;

    println!("Sending a ping message.");
    ws.send(websockets::Frame::Ping { payload: None }).await?;

    print!("Waiting on a response: ");
    let frame = ws.receive().await?;
    match frame {
        Frame::Pong { .. } => println!("got a pong message."),
        _ => println!("got a non-Pong message."),
    }

    println!("Closing WebSocket.");
    ws.close(None).await?;

    Ok(())
}
