use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use uuid::Uuid;

use crate::{
    db::MazeRepository,
    entities::{CreateMazeRequest, MazeResponse, MazeSolutionRequest, MazeSolutionResponse},
    errors::AppError,
};

type ApiResult<T> = Result<Json<T>, AppError>;

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
    Path(id): Path<Uuid>,
) -> ApiResult<MazeResponse> {
    repository
        .get_by_id(id)
        .map(Json)
        .ok_or_else(|| AppError::not_found("maze not found"))
}

async fn get_all_mazes(
    State(repository): State<MazeRepository>,
) -> ApiResult<Vec<MazeResponse>> {
    Ok(Json(repository.get_all()))
}

async fn get_maze_solution(
    State(repository): State<MazeRepository>,
    Path(id): Path<Uuid>,
    Json(request): Json<MazeSolutionRequest>,
) -> ApiResult<MazeSolutionResponse> {
    let maze = repository
        .get_by_id(id)
        .ok_or_else(|| AppError::not_found("maze not found"))?;

    let mut map = crate::domain::Map::parse_from_string(&maze.content)
        .map_err(|e| AppError::bad_request(format!("invalid maze: {}", e)))?;

    let player_pos = crate::domain::Position {
        row: request.player_row,
        col: request.player_col,
    };
    let portal_pos = crate::domain::Position {
        row: request.portal_row,
        col: request.portal_col,
    };

    if player_pos.row >= map.rows || player_pos.col >= map.cols {
        return Err(AppError::bad_request("invalid player coordinates"));
    }
    if portal_pos.row >= map.rows || portal_pos.col >= map.cols {
        return Err(AppError::bad_request("invalid portal coordinates"));
    }

    map.start = player_pos;
    map.end = portal_pos;

    let path = crate::domain::find_path(&map)
        .ok_or_else(|| AppError::bad_request("no path found"))?;

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
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if repository.delete(id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found("maze not found"))
    }
}

async fn create_maze(
    State(repository): State<MazeRepository>,
    Json(request): Json<CreateMazeRequest>,
) -> ApiResult<MazeResponse> {
    crate::domain::Map::parse_from_string(&request.content)
        .map_err(|e| AppError::bad_request(format!("invalid maze: {}", e)))?;

    Ok(Json(repository.create(&request.name, &request.content)))
}

