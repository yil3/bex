use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Option<String>,
    pub owner: Option<String>,
    pub account: Option<String>,
    pub nickname: Option<String>,
    pub lastname: Option<String>,
    pub type_of: Option<String>,
    pub passwd: Option<String>,
    pub passwd_salt: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub address: Option<String>,
    pub id_card: Option<String>,

    pub signup_application: Option<String>,

    pub qq: Option<String>,
    pub wechat: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
    pub last_signin_date: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserInfo {
    pub id: Option<String>,
    #[sqlx(default)]
    pub owner: Option<String>,
    pub account: Option<String>,
    pub nickname: Option<String>,
    #[sqlx(default)]
    pub type_of: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub address: Option<String>,
    #[sqlx(default)]
    pub id_card: Option<String>,

    pub qq: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}
