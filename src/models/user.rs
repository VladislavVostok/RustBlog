use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String
}

impl User{
    pub async fn create(
        username: &str,
        email: &str,
        password: &str,
        pool: &sqlx::MySqlPool
    ) -> Result<Self, sqlx::Error> {

        let password_hash = hash(password, DEFAULT_COST).unwrap();

        sqlx::query_as::<_, User>("INSERT INTO users(username, email, password_hash) VALUES ($1, $2, $3) RETURNING *")
            .bind(username)
            .bind(email)
            .bind(password_hash)
            .fetch_one(pool)
            .await
    }

    pub async fn find_by_username(
        username: &str,
        pool: &sqlx::MySqlPool
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await
    }

    pub fn verify_password(
        &self,
        password: &str
    ) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }

}