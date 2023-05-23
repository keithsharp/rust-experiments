use std::error::Error;
use std::fmt::Display;

use axum::extract::rejection::TypedHeaderRejection;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{self, FromRequestParts, TypedHeader, WebSocketUpgrade};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::http::StatusCode;
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

// Keys for encoding and decoding JWTs
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

// Claims that are encoded in the JWT
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
    type Rejection = ClaimError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let bearer = match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
            Ok(TypedHeader(Authorization(bearer))) => bearer,
            Err(e) => return Err(ClaimError::HeaderFormatError { source: e }),
        };

        let token_data =
            match decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default()) {
                Ok(token_data) => token_data,
                Err(e) => return Err(ClaimError::TokenError { source: e }),
            };

        Ok(token_data.claims)
    }
}

// Errors that can be encountered when decoding an Authorization header
#[derive(Debug)]
pub enum ClaimError {
    HeaderFormatError { source: TypedHeaderRejection },
    TokenError { source: jsonwebtoken::errors::Error },
}

impl Display for ClaimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClaimError::HeaderFormatError { source } => {
                write!(f, "Error with Authorization header: {}", source)
            }
            ClaimError::TokenError { source } => write!(f, "Invalid token error: {}", source),
        }
    }
}

impl IntoResponse for ClaimError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ClaimError::HeaderFormatError { .. } => (StatusCode::BAD_REQUEST, self).into_response(),
            ClaimError::TokenError { .. } => (StatusCode::UNAUTHORIZED, self).into_response(),
        }
    }
}

impl Error for ClaimError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ClaimError::HeaderFormatError { source } => Some(source),
            ClaimError::TokenError { source } => Some(source),
        }
    }
}

// The main function
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

// Check username and password and return a JWT
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

// Upgrade to a WebSocket
async fn websocket_upgrade(claims: Claims, ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("Got a connection, upgrading to a WebSocket.");
    ws.on_upgrade(|socket| websocket_handler(socket, claims))
}

// Process messages on the WekSocket
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
