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

pub async fn authenticate_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<i32, DataError> {
    //
    // query using function 👇

    // let query = "SELECT id, password_hash FROM users WHERE email = $1";

    // let row = sqlx::query(query)
    //     .bind(email)
    //     .fetch_one(pool)
    //     .await
    //     .map_err(|e| match e {
    //         sqlx::Error::RowNotFound => DataError::FailedQuery("User Not found".into()),
    //         e => DataError::Query(e),
    //     })?;

    // let password_hash: Vec<u8> = row.try_get("password_hash")?;

    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => DataError::FailedQuery("Invalid credentials".to_string()),
        e => DataError::Query(e),
    })?;

    let hashed_password = String::from_utf8(user.password_hash)?;

    let valid_password = bcrypt::verify(password, &hashed_password)?;

    if !valid_password {
        Err(DataError::FailedQuery("Invalid credentials".to_string()))
    } else {
        Ok(user.id)
    }
}
