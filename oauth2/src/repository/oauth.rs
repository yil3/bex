use crate::POOL;
use crate::models::user::UserInfo;
use sqlx::Error;

pub struct Dao;

impl Dao {
    pub async fn sign_in(&self, account: &str, passwd: &str) -> Result<UserInfo, Error> {
        sqlx::query_as::<_, UserInfo>(
            "SELECT * FROM sys_user where account = $1 and passwd = $2;",
        )
        .bind(account)
        .bind(passwd)
        .fetch_one(&*POOL)
        .await
    }
}
