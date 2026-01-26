use axum::{Extension, Json, http::StatusCode, response::{IntoResponse, Response}};
use models::{db::User, rest::{UserRegister}};
use sqlx::{Pool, Postgres};

pub async fn register_user(
    Extension(conn): Extension<Pool<Postgres>>,
    Json(inp): Json<UserRegister>,
) -> Response
{
    let x = sqlx::query_as::<_, User>("SELECT * from users where username = $1 and password_hash = encode(sha256($2::bytea), 'hex'); ")
    .bind(&inp.username)
    .bind(&inp.password)
    .fetch_one(&conn)
    .await;

    return match x {
        Ok(user) => {
            (StatusCode::UNAUTHORIZED, "User Already Exists").into_response()
        },
        Err(sqlx::Error::RowNotFound) => {
            let res = sqlx::query(r#"
            INSERT INTO users (user_id, username, password_hash, created_at) 
            values (
                gen_random_uuid(), 
                $1, 
                encode(sha256($2::bytea), 'hex'), CURRENT_TIMESTAMP);
            "#)
            .bind(&inp.username)
            .bind(&inp.password)
            .execute(&conn)
            .await;

            match res {
                Ok(result) => {},
                Err(err) => {
                    eprintln!("{:?}", err); // TODO
                }
            }

            (StatusCode::CREATED).into_response()
        },
        Err(err) => {
            (StatusCode::UNAUTHORIZED, err.to_string()).into_response()
        }
    }
}

pub async fn login_user(
    Extension(conn): Extension<Pool<Postgres>>,
    Json(inp): Json<UserRegister>,
) {
    
}

