use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use crate::entities::{CreateMazeRequest, MazeResponse, MazeSolutionRequest, MazeSolutionResponse};
use crate::db::MazeRepository;
use anyhow::Result;

pub fn create_router(repository: MazeRepository) -> Router {
    Router::new()
        .route("/api/mazes/:id", get(get_maze))
        .route("/api/mazes/:id", delete(delete_maze))
        .route("/api/mazes/:id/solution", post(get_maze_solution))
        .route("/api/mazes", get(get_all_mazes))
        .route("/api/mazes", post(create_maze))
        .with_state(repository)
}

async fn get_maze(
    State(repository): State<MazeRepository>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<MazeResponse>, (StatusCode, String)> {
    match repository.get_by_id(id).await {
        Ok(Some(maze)) => Ok(Json(maze)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Maze not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_all_mazes(
    State(repository): State<MazeRepository>,
) -> Result<Json<Vec<MazeResponse>>, (StatusCode, String)> {
    match repository.get_all().await {
        Ok(mazes) => Ok(Json(mazes)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_maze_solution(
    State(repository): State<MazeRepository>,
    Path(id): Path<uuid::Uuid>,
    Json(request): Json<MazeSolutionRequest>,
) -> Result<Json<MazeSolutionResponse>, (StatusCode, String)> {
    let maze = match repository.get_by_id(id).await {
        Ok(Some(m)) => m,
        Ok(None) => return Err((StatusCode::NOT_FOUND, "Maze not found".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    let mut map = crate::domain::Map::parse_from_string(&maze.content)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid maze format: {}", e)))?;

    let player_pos = crate::domain::Position { row: request.player_row, col: request.player_col };
    let portal_pos = crate::domain::Position { row: request.portal_row, col: request.portal_col };

    if player_pos.row >= map.rows || player_pos.col >= map.cols {
        return Err((StatusCode::BAD_REQUEST, "Invalid player coordinates".to_string()));
    }
    if portal_pos.row >= map.rows || portal_pos.col >= map.cols {
        return Err((StatusCode::BAD_REQUEST, "Invalid portal coordinates".to_string()));
    }

    map.start = player_pos;
    map.end = portal_pos;

    let path = crate::domain::find_path(&map)
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "No path found".to_string()))?;

    map.mark_path(&path);
    let solution = map.to_string();

    Ok(Json(MazeSolutionResponse {
        id: maze.id,
        name: maze.name,
        solution,
    }))
}

async fn delete_maze(
    State(repository): State<MazeRepository>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    match repository.delete(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((StatusCode::NOT_FOUND, "Maze not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn create_maze(
    State(repository): State<MazeRepository>,
    Json(request): Json<CreateMazeRequest>,
) -> Result<Json<MazeResponse>, (StatusCode, String)> {
    crate::domain::Map::parse_from_string(&request.content)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid maze format: {}", e)))?;
    
    match repository.create(&request.name, &request.content).await {
        Ok(maze) => Ok(Json(maze)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

