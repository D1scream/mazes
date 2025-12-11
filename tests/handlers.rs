use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use mazes::{db::MazeRepository, handlers};
use tower::ServiceExt;

#[tokio::test]
async fn test_create_maze() {
    let app = handlers::create_router(MazeRepository::new());

    let request = Request::builder()
        .method("POST")
        .uri("/api/mazes")
        .header("content-type", "application/json")
        .body(Body::from("{\"name\":\"Test Maze\",\"content\":\"###\\n#iO\\n###\"}"))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_maze() {
    let app = handlers::create_router(MazeRepository::new());

    let create_request = Request::builder()
        .method("POST")
        .uri("/api/mazes")
        .header("content-type", "application/json")
        .body(Body::from("{\"name\":\"Test Maze\",\"content\":\"###\\n#iO\\n###\"}"))
        .unwrap();

    let create_response = app.clone().oneshot(create_request).await.unwrap();
    assert_eq!(create_response.status(), StatusCode::OK);

    let body = create_response.into_body().collect().await.unwrap().to_bytes();
    let maze: mazes::entities::MazeResponse = serde_json::from_slice(&body).unwrap();

    let get_request = Request::builder()
        .method("GET")
        .uri(&format!("/api/mazes/{}", maze.id))
        .body(Body::empty())
        .unwrap();

    let get_response = app.oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_all_mazes() {
    let app = handlers::create_router(MazeRepository::new());

    let request = Request::builder()
        .method("GET")
        .uri("/api/mazes")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_maze() {
    let app = handlers::create_router(MazeRepository::new());

    let create_request = Request::builder()
        .method("POST")
        .uri("/api/mazes")
        .header("content-type", "application/json")
        .body(Body::from("{\"name\":\"Test Maze\",\"content\":\"###\\n#iO\\n###\"}"))
        .unwrap();

    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let body = create_response.into_body().collect().await.unwrap().to_bytes();
    let maze: mazes::entities::MazeResponse = serde_json::from_slice(&body).unwrap();

    let delete_request = Request::builder()
        .method("DELETE")
        .uri(&format!("/api/mazes/{}", maze.id))
        .body(Body::empty())
        .unwrap();

    let delete_response = app.clone().oneshot(delete_request).await.unwrap();
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    let get_request = Request::builder()
        .method("GET")
        .uri(&format!("/api/mazes/{}", maze.id))
        .body(Body::empty())
        .unwrap();

    let get_response = app.oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_maze_solution() {
    let app = handlers::create_router(MazeRepository::new());

    let create_request = Request::builder()
        .method("POST")
        .uri("/api/mazes")
        .header("content-type", "application/json")
        .body(Body::from("{\"name\":\"Test Maze\",\"content\":\"###\\n#iO\\n###\"}"))
        .unwrap();

    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let body = create_response.into_body().collect().await.unwrap().to_bytes();
    let maze: mazes::entities::MazeResponse = serde_json::from_slice(&body).unwrap();

    let solution_request = Request::builder()
        .method("POST")
        .uri(&format!("/api/mazes/{}/solution", maze.id))
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"player_row":1,"player_col":1,"portal_row":1,"portal_col":2}"#,
        ))
        .unwrap();

    let solution_response = app.oneshot(solution_request).await.unwrap();
    assert_eq!(solution_response.status(), StatusCode::OK);
}
