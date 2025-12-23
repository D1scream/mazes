use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MazeResponse {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

