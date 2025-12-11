#[cfg(test)]
mod tests {
    use crate::{db::MazeRepository, handlers};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;
    use http_body_util::BodyExt;

    async fn setup_test_db() -> MazeRepository {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/pacwoman_test".to_string());
        
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        sqlx::query("TRUNCATE TABLE mazes").execute(&pool).await.ok();

        MazeRepository::new(pool)
    }

    #[tokio::test]
    async fn test_create_maze() {
        let repository = setup_test_db().await;
        let app = handlers::create_router(repository);

        let request = Request::builder()
            .method("POST")
            .uri("/api/mazes")
            .header("content-type", "application/json")
            .body(Body::from("{\"name\":\"Test Maze\",\"content\":\"##\\n#i#\\n#O#\"}"))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_maze() {
        let repository = setup_test_db().await;
        let app = handlers::create_router(repository.clone());

        let create_request = Request::builder()
            .method("POST")
            .uri("/api/mazes")
            .header("content-type", "application/json")
            .body(Body::from("{\"name\":\"Test Maze\",\"content\":\"##\\n#i#\\n#O#\"}"))
            .unwrap();

        let create_response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(create_response.status(), StatusCode::OK);

        let body = create_response.into_body().collect().await.unwrap().to_bytes();
        let maze: crate::entities::MazeResponse = serde_json::from_slice(&body).unwrap();

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
        let repository = setup_test_db().await;
        let app = handlers::create_router(repository);

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
        let repository = setup_test_db().await;
        let app = handlers::create_router(repository.clone());

        let create_request = Request::builder()
            .method("POST")
            .uri("/api/mazes")
            .header("content-type", "application/json")
            .body(Body::from("{\"name\":\"Test Maze\",\"content\":\"##\\n#i#\\n#O#\"}"))
            .unwrap();

        let create_response = app.clone().oneshot(create_request).await.unwrap();
        let body = create_response.into_body().collect().await.unwrap().to_bytes();
        let maze: crate::entities::MazeResponse = serde_json::from_slice(&body).unwrap();

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
        let repository = setup_test_db().await;
        let app = handlers::create_router(repository.clone());

        let create_request = Request::builder()
            .method("POST")
            .uri("/api/mazes")
            .header("content-type", "application/json")
            .body(Body::from("{\"name\":\"Test Maze\",\"content\":\"   \\n#i#\\n#O#\"}"))
            .unwrap();

        let create_response = app.clone().oneshot(create_request).await.unwrap();
        let body = create_response.into_body().collect().await.unwrap().to_bytes();
        let maze: crate::entities::MazeResponse = serde_json::from_slice(&body).unwrap();

        let solution_request = Request::builder()
            .method("POST")
            .uri(&format!("/api/mazes/{}/solution", maze.id))
            .header("content-type", "application/json")
            .body(Body::from(r#"{"player_row":1,"player_col":1,"portal_row":2,"portal_col":1}"#))
            .unwrap();

        let solution_response = app.oneshot(solution_request).await.unwrap();
        assert_eq!(solution_response.status(), StatusCode::OK);
    }
}

