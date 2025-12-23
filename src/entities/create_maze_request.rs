use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMazeRequest {
    pub name: String,
    pub content: String,
}

