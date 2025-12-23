use crate::entities::MazeResponse;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct MazeRepository {
    pool: PgPool,
}

impl MazeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, name: &str, content: &str) -> Result<MazeResponse, sqlx::Error> {
        let id = Uuid::new_v4();
        let maze = sqlx::query_as!(
            MazeResponse,
            r#"
            INSERT INTO mazes (id, name, content)
            VALUES ($1, $2, $3)
            RETURNING id, name, content, created_at
            "#,
            id,
            name,
            content
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(maze)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<MazeResponse>, sqlx::Error> {
        let maze = sqlx::query_as!(
            MazeResponse,
            r#"
            SELECT id, name, content, created_at
            FROM mazes
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(maze)
    }

    pub async fn get_all(&self) -> Result<Vec<MazeResponse>, sqlx::Error> {
        let mazes = sqlx::query_as!(
            MazeResponse,
            r#"
            SELECT id, name, content, created_at
            FROM mazes
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(mazes)
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM mazes
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
