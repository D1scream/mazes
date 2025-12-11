use mazes::{db::MazeRepository, handlers};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repository = MazeRepository::new();
    let app = handlers::create_router(repository);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
