use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MazeSolutionRequest {
    pub player_row: usize,
    pub player_col: usize,
    pub portal_row: usize,
    pub portal_col: usize,
}

