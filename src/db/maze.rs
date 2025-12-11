use sqlx::PgPool;
use crate::entities::{Maze, MazeResponse};
use anyhow::{Result, Context};

#[derive(Clone)]
pub struct MazeRepository {
    pool: PgPool,
}

impl MazeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, name: &str, content: &str) -> Result<MazeResponse> {
        let maze = sqlx::query_as::<_, Maze>(
            "INSERT INTO mazes (name, content) VALUES ($1, $2) RETURNING id, name, content, created_at"
        )
        .bind(name)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;

        Ok(MazeResponse {
            id: maze.id.context("Maze ID should be present after insert")?,
            name: maze.name,
            content: maze.content,
            created_at: maze.created_at.context("created_at should be present after insert")?,
        })
    }

    pub async fn get_by_id(&self, id: uuid::Uuid) -> Result<Option<MazeResponse>> {
        let maze = sqlx::query_as::<_, Maze>(
            "SELECT id, name, content, created_at FROM mazes WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match maze {
            Some(m) => Ok(Some(MazeResponse {
                id: m.id.context("Maze ID should be present")?,
                name: m.name,
                content: m.content,
                created_at: m.created_at.context("created_at should be present")?,
            })),
            None => Ok(None),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<MazeResponse>> {
        let mazes = sqlx::query_as::<_, Maze>(
            "SELECT id, name, content, created_at FROM mazes ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for m in mazes {
            result.push(MazeResponse {
                id: m.id.context("Maze ID should be present")?,
                name: m.name,
                content: m.content,
                created_at: m.created_at.context("created_at should be present")?,
            });
        }

        Ok(result)
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<bool> {
        let rows_affected = sqlx::query("DELETE FROM mazes WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(rows_affected.rows_affected() > 0)
    }
}

