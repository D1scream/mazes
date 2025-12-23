use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MazeSolutionResponse {
    pub id: Uuid,
    pub name: String,
    pub solution: String,
}

