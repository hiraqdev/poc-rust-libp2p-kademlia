use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GreeRequest {
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GreetResponse {
    pub message: String
}