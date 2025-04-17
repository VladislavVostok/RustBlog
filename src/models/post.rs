use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {
    pub async fn all(pool: &sqlx::MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Post>("SELECT id, title, body, published FROM posts")
            .fetch_all(pool)
            .await
    }

    pub async fn find(id: u64, pool: &sqlx::MySqlPool) -> Result<Self, sqlx::Error> {  // Изменили возвращаемый тип и тип параметра
        sqlx::query_as::<_, Post>("SELECT id, title, body, published FROM posts WHERE id = ?")
            .bind(id)
            .fetch_one(pool)  // Используем fetch_one вместо fetch_all
            .await
    }
}