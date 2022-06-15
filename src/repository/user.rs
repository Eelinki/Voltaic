use sqlx::{SqlitePool};
use crate::model::{Session, User};
use crate::http::{Error, Result};

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM strapi_administrator")
        .fetch_all(pool)
        .await.unwrap();

    let mut vec = Vec::new();

    for row in rows {
        vec.push(User {
            id: row.id,
            firstname: row.firstname.unwrap(),
            lastname: row.lastname.unwrap(),
            email: row.email,
            password: row.password.unwrap(),
        })
    }

    Ok(vec)
}

pub async fn by_email(pool: &SqlitePool, email: String) -> Result<User> {
    let row = sqlx::query!("SELECT * FROM strapi_administrator WHERE email = ?", email)
        .fetch_optional(pool)
        .await?
        .ok_or(Error::NotFound)?;

    let user = User {
        id: row.id,
        firstname: row.firstname.unwrap(),
        lastname: row.lastname.unwrap(),
        email: row.email,
        password: row.password.unwrap(),
    };

    Ok(user)
}

pub async fn create_session(pool: &SqlitePool, user: &User, session_id: Vec<u8>) -> Result<Session> {
    sqlx::query!("DELETE FROM session WHERE user_id = ?", user.id).execute(pool).await?;

    let session = sqlx::query_as!(Session, "INSERT INTO session (id, user_id) VALUES (?, ?) RETURNING id, user_id", session_id, user.id)
        .fetch_optional(pool)
        .await?
        .ok_or(Error::NotFound)?;

    Ok(session)
}


pub async fn verify_session(pool: &SqlitePool, session_id: Vec<u8>) -> Result<bool> {
    let row = sqlx::query!("SELECT * FROM session WHERE id = ?", session_id).fetch_optional(pool).await?;

    if row.is_none() {
        return Ok(false)
    }

    Ok(true)
}

pub async fn destroy_session(pool: &SqlitePool, session_id: Vec<u8>) -> Result<bool> {
    sqlx::query!("DELETE FROM session WHERE id = ?", session_id).execute(pool).await?;

    Ok(true)
}