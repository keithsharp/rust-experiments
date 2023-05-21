use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MyRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MyResponse {
    pub message: String,
}
