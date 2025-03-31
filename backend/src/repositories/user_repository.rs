// use sqlx::PgPool;
// use crate::models::User;

// pub async fn create_user(
//     pool: &PgPool,
//     username: &str,
//     password: &str,
//     role: &str,
// ) -> Result<(), sqlx::Error> {
//     sqlx::query!(
//         "INSERT INTO users (username, password, role) VALUES ($1, $2, $3)",
//         username,
//         password,
//         role
//     )
//     .execute(pool)
//     .await?;
//     Ok(())
// }

// pub async fn find_user_by_username(
//     pool: &PgPool,
//     username: &str,
// ) -> Result<User, sqlx::Error> {
//     let user = sqlx::query_as!(
//         User,
//         "SELECT id, username, password, role FROM users WHERE username = $1",
//         username
//     )
//     .fetch_one(pool)
//     .await?;
//     Ok(user)
// }
