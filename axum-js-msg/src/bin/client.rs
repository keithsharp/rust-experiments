use websockets::{Frame, WebSocket};

use axum_js_msg::{MyRequest, MyResponse};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ws = WebSocket::connect("ws://127.0.0.1:3000/websocket").await?;

    println!("Sending a message.");
    let message = MyRequest {
        name: "Bob".to_string(),
    };
    ws.send_text(serde_json::to_string(&message)?).await?;

    print!("Waiting on a response: ");
    let frame = ws.receive().await?;
    match frame {
        Frame::Pong { .. } => println!("got a pong message."),
        Frame::Text { payload, .. } => {
            let response: MyResponse = serde_json::from_str(&payload)?;
            println!("Got response message with message: '{}'.", response.message);
        }
        _ => println!("got a non-Pong message."),
    }

    println!("Closing WebSocket.");
    ws.close(None).await?;

    Ok(())
}
