//! This module keeps the types of structs that are returned in the JSON response bodies.
use serde::Serialize;

/// Returned whenever an error occured and the request could not be completed.
#[derive(Serialize)]
pub struct MessageResponse {
    message: String,
}

impl MessageResponse {
    pub fn new(message: &str) -> Self {
        MessageResponse {
            message: String::from(message),
        }
    }
}
