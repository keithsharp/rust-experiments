use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub const FAKE_EMAIL: &'static str = "user@example.com";
pub const FAKE_PASSWORD: &'static str = "password1234";

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

impl Display for AuthRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: '{}', Password: '{}'", self.email, self.password)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub r#type: String,
}

impl Display for AuthResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.r#type, self.token)
    }
}
