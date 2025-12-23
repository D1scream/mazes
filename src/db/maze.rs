use crate::entities::MazeResponse;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct MazeRepository {
    store: Arc<Mutex<Vec<MazeResponse>>>,
}

impl MazeRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(&self, name: &str, content: &str) -> MazeResponse {
        let maze = MazeResponse {
            id: Uuid::new_v4(),
            name: name.to_owned(),
            content: content.to_owned(),
            created_at: Utc::now(),
        };

        let mut store = self.store.lock().expect("store poisoned");
        store.push(maze.clone());
        maze
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<MazeResponse> {
        let store = self.store.lock().expect("store poisoned");
        store.iter().find(|m| m.id == id).cloned()
    }

    pub fn get_all(&self) -> Vec<MazeResponse> {
        let store = self.store.lock().expect("store poisoned");
        let mut mazes: Vec<_> = store.iter().cloned().collect();
        mazes.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        mazes
    }

    pub fn delete(&self, id: Uuid) -> bool {
        let mut store = self.store.lock().expect("store poisoned");
        let initial_len = store.len();
        store.retain(|m| m.id != id);
        store.len() != initial_len
    }
}
