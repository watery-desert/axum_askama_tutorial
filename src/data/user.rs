use super::errors::DataError;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<(), DataError> {
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    let bytea_hash = hashed_password.as_bytes();

    // query using function 👇

    //     let query = r#"
    //     INSERT INTO users(email, password_hash)
    //     VALUES($1, $2)"#;

    //    sqlx::query(&query)
    //         .bind(email)
    //         .bind(bytea_hash)
    //         .execute(pool)
    //         .await?;

    sqlx::query!(
        "INSERT INTO users(email, password_hash) 
    VALUES($1, $2)",
        email,
        bytea_hash
    )
    .execute(pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(e) => {
            if e.constraint() == Some("users_email_key") {
                DataError::FailedQuery("This email address is already used".to_string())
            } else {
                DataError::Internal(e.to_string())
            }
        }
        e => DataError::Query(e),
    })?;

    Ok(())

    // Simulate Internal server error 👇
    // Err(DataError::Internal("Test error".to_string()))
}
