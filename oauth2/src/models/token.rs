use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::user::UserInfo;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Token {
    pub id: Option<String>,
    pub owner: Option<String>,
    pub appliction: Option<String>,
    pub organization: Option<String>,
    pub code: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub exp: Option<i64>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenWrap {
    pub access_token: String,
    pub refresh_token: String,
    pub exp: usize,
    pub scope: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user: UserInfo,
    pub exp: usize,
}


#[derive(Debug)]
pub enum AuthError {
    Auth,
    Internal,
}

