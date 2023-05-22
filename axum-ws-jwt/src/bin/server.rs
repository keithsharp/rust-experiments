use axum::extract::ws::{Message, WebSocket};
use axum::extract::{self, FromRequestParts, TypedHeader, WebSocketUpgrade};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{async_trait, Json, RequestPartsExt, Router};

use futures::{sink::SinkExt, stream::StreamExt};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use axum_ws_jwt::{AuthRequest, AuthResponse};

const JWT_SECRET: &'static str = "JWT Secret";
const JWT_VALID_DAYS: i64 = 7;

static KEYS: Lazy<Keys> = Lazy::new(|| Keys::new(JWT_SECRET));

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| "Invalid Authorization header format".to_string())?;
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| "Invalid token".to_string())?;
        Ok(token_data.claims)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/login", post(login_handler))
        .route("/websocket", get(websocket_upgrade));

    println!("Starting server on '127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn login_handler(
    extract::Json(auth): extract::Json<AuthRequest>,
) -> Result<Json<AuthResponse>, String> {
    print!("Got a login request... ");
    if auth.email != axum_ws_jwt::FAKE_EMAIL || auth.password != axum_ws_jwt::FAKE_PASSWORD {
        println!("Incorrect credentials: {}.", auth);
        return Err("Incorrect credentials".to_string());
    }

    let expiry = chrono::Utc::now() + chrono::Duration::days(JWT_VALID_DAYS);
    let claims = Claims {
        sub: auth.email,
        exp: expiry.timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding).unwrap();
    let response = AuthResponse {
        token: token,
        r#type: "Bearer".to_string(),
    };
    println!("Replying with JWT.");

    Ok(Json(response))
}

async fn websocket_upgrade(claims: Claims, ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("Got a connection, upgrading to a WebSocket.");
    ws.on_upgrade(|socket| websocket_handler(socket, claims))
}

async fn websocket_handler(stream: WebSocket, claims: Claims) {
    let (mut ws_tx, mut ws_rx) = stream.split();

    // Would use the values from 'claims' to authorise actions
    // rather than printing them out...
    println!(
        "Listening for messages on the WebSocket for user '{}'.",
        &claims.sub
    );

    while let Some(message) = ws_rx.next().await {
        match message {
            Ok(message) => match message {
                Message::Ping(_) => {
                    println!("Got a ping message, sending a pong.");
                    ws_tx.send(Message::Pong(Vec::new())).await.unwrap()
                }
                Message::Close(_) => {
                    println!("Got a connection close message.");
                    break;
                }
                m => println!("Got an unimplemented message: {:?}", m),
            },
            Err(e) => eprintln!("Error reading message from WebSocket: {}", e),
        }
    }
    println!("Handler exiting.");
}
