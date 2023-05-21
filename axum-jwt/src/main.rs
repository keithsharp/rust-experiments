use std::fmt::Display;

use axum::extract::TypedHeader;
use axum::extract::{self, FromRequestParts};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::routing::{get, post};
use axum::RequestPartsExt;
use axum::{async_trait, Json, Router};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

const JWT_SECRET: &'static str = "JWT Secret";
const JWT_VALID_DAYS: i64 = 7;
const FAKE_EMAIL: &'static str = "user@example.com";
const FAKE_PASSWORD: &'static str = "password1234";

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

#[derive(Debug, Deserialize)]
struct AuthRequest {
    email: String,
    password: String,
}

impl Display for AuthRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: '{}', Password: '{}'", self.email, self.password)
    }
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: String,
    r#type: String,
}

impl Display for AuthResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.r#type, self.token)
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
        .route("/private", get(private_handler));

    println!("Starting server on '127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn login_handler(
    extract::Json(auth): extract::Json<AuthRequest>,
) -> Result<Json<AuthResponse>, String> {
    if auth.email != FAKE_EMAIL || auth.password != FAKE_PASSWORD {
        println!("Incorrect credentials: {}", auth);
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
        r#type: "BEARER".to_string(),
    };

    Ok(Json(response))
}

async fn private_handler(claims: Claims) -> String {
    println!("{:?}", claims);
    "You have been permitted access".to_string()
}
