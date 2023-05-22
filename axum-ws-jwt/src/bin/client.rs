use websockets::{Frame, WebSocket};

use axum_ws_jwt::{AuthRequest, AuthResponse};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let request = AuthRequest {
        email: axum_ws_jwt::FAKE_EMAIL.to_string(),
        password: axum_ws_jwt::FAKE_PASSWORD.to_string(),
    };

    println!("Authenticating to server.");
    let response: AuthResponse = reqwest::Client::new()
        .post("http://127.0.0.1:3000/login")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    println!("Authorization: '{}'", &response);

    println!("Connecting WebSocket.");
    let mut ws = WebSocket::builder()
        .add_header("Authorization", response.to_string().as_str())
        .connect("ws://127.0.0.1:3000/websocket")
        .await?;

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
