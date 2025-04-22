use sqlx::{FromRow, Encode,};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::Value;

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub enum UserRole{
//     #[serde(rename = "user")]
//     User,
//     #[serde(rename = "moderator")]
//     Moderator,
//     #[serde(rename = "admin")]
//     Admin,
// }

// impl sqlx::Type<sqlx::MySql> for UserRole{
//     fn type_info() -> sqlx::mysql::MySqlTypeInfo {
//         <str as sqlx::Type<sqlx::MySql>>::type_info()
//     }
// }
//
// impl<'r> sqlx::Decode<'r, sqlx::MySql> for UserRole{
//     fn decode(
//         value: sqlx::mysql::MySqlValueRef<'r>,
//     ) ->Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>>{
//         let s = <&str as sqlx::Decode<sqlx::MySql>>::decode(value)?;
//         match s {
//             "user" => Ok(UserRole::User),
//             "moderator" => Ok(UserRole::Moderator),
//             "admin" => Ok(UserRole::Admin),
//             _ => Err(format!("unknown role {}", s).into()),
//         }
//
//     }
// }
//
// impl<'q> Encode<'q, MySql> for UserRole {
//     fn encode_by_ref(
//         &self,
//         buf: &mut <MySql as HasArguments<'q>>::ArgumentBuffer,
//     ) -> IsNull {
//         let bytes: &[u8] = &self.0.to_base62().into_bytes();
//         <&[u8] as Encode<MySql>>::encode(bytes, buf)
//     }
// }


#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    // pub role: UserRole,
    #[serde(skip_serializing)]
    pub password_hash: String
}



impl User{
    pub async fn create(
        username: &str,
        email: &str,
        password: &str,
        // role: UserRole,
        pool: &sqlx::MySqlPool
    ) -> Result<Self, sqlx::Error> {

        let password_hash = hash(password, DEFAULT_COST).unwrap();

        // sqlx::query_as::<_, User>("INSERT INTO users(username, email, password_hash, role) VALUES ($1, $2, $3, $4) RETURNING *")
        sqlx::query_as::<_, User>("INSERT INTO users(username, email, password_hash) VALUES ($1, $2, $3) RETURNING *")
            .bind(username)
            .bind(email)
            .bind(password_hash)
            // .bind(role)
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