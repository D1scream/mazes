use mazes::{db::MazeRepository, handlers};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/mazes".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    println!("Migrations completed successfully");

    let repository = MazeRepository::new(pool);
    let app = handlers::create_router(repository);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
