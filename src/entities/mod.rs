use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMazeRequest {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MazeResponse {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MazeSolutionRequest {
    pub player_row: usize,
    pub player_col: usize,
    pub portal_row: usize,
    pub portal_col: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MazeSolutionResponse {
    pub id: Uuid,
    pub name: String,
    pub solution: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}
