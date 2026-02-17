use axum::{
    Extension, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use models::{
    db::User,
    rest::{Claim, LoginResponse, UserLogin, UserRegister},
};
use serde_valid::Validate;
use sqlx::{Pool, Postgres, error::ErrorKind};


pub async fn register_user(
    Extension(conn): Extension<Pool<Postgres>>,
    Json(inp): Json<UserRegister>,
) -> Response {
    let validation_result = inp.validate();
    if validation_result.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(validation_result.unwrap_err())
        ).into_response()
    }

    let res = sqlx::query(
        r#"
        INSERT INTO users (user_id, username, password_hash, created_at) 
        values (
            gen_random_uuid(), 
            $1, 
            encode(sha256($2::bytea), 'hex'), CURRENT_TIMESTAMP);
    "#,
    )
    .bind(&inp.username)
    .bind(&inp.password)
    .execute(&conn)
    .await;

    match res {
        Ok(result) => {
            (StatusCode::CREATED).into_response()
        }
        Err(err) => {
            match err.as_database_error().unwrap().kind() { // TODO
                ErrorKind::UniqueViolation => {
                    (StatusCode::ALREADY_REPORTED, "User Already Exists".to_string()).into_response()
                },
                err_kind => {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("{:?}: {:?}",err_kind, err)
                    ).into_response()
                }
            }
        }
    }
}

pub async fn login_user(
    Extension(conn): Extension<Pool<Postgres>>,
    Json(body): Json<UserLogin>,
) -> Response {
    if body.username.is_empty() || body.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            "username and password shouldn't be empty".to_string(),
        )
            .into_response();
    };

    let result = sqlx::query_as::<_,User>("select * from users where username = $1 and password_hash = encode(sha256($2::bytea), 'hex')")
    .bind(&body.username)
    .bind(&body.password)
    .fetch_optional(&conn)
    .await;

    match result {
        Ok(res) => {
            if let Some(user) = res {
                let claim = Claim {
                    sub: user.username.clone(),
                    exp: (Utc::now() + Duration::seconds(60)).timestamp() as usize,
                };
                match encode(
                    &Header::default(),
                    &claim,
                    &EncodingKey::from_secret(&"secret".as_ref()),
                ) {
                    Ok(tok) => (StatusCode::OK, Json(LoginResponse { token: tok })).into_response(),
                    Err(err) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Error generating token: {:?}", err),
                    )
                        .into_response(),
                }
            } else {
                (StatusCode::NOT_FOUND, "username or password are incorrect").into_response()
            }
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {:?}", err),
        )
            .into_response(),
    }
}

pub async fn validate_user_creds(inp: UserRegister) {}
