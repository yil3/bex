use std::collections::HashMap;

use crate::models::user::{User, UserInfo};
use crate::POOL;
use common::utils;
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, Row};

pub struct Dao;

impl Dao {
    pub async fn insert(&self, user: User) -> Result<PgQueryResult, Error> {
        sqlx::query(
            "
            insert into sys_user 
            (id, account, passwd) 
            values 
            ($1, $2, $3)",
        )
        .bind(utils::code::unique_id())
        .bind(user.account)
        .bind(user.passwd)
        .execute(&*POOL)
        .await
    }

    pub async fn update(&self, user: User) -> Result<PgQueryResult, Error> {
        sqlx::query!(
            "
            update sys_user set 
            email = coalesce($2, email),
            address = coalesce($3, address)
            where id = $1
            ",
            user.id,
            user.email,
            user.address
        )
        .execute(&*POOL)
        .await
    }

    pub async fn page(
        &self,
        limit: u32,
        offset: u32,
        params: &HashMap<String, String>,
    ) -> (Vec<UserInfo>, i64) {
        let mut sql = String::from("select * from sys_user where 1=1");
        let mut count_sql = String::from("select count(*) from sys_user where 1=1");
        if let Some(query) = params.get("query") {
            if !query.is_empty() {
                let query = &format!(" and nickname like '%{query}%'");
                sql.push_str(query);
                count_sql.push_str(query);
            }
        }
        sql.push_str(" order by create_at desc");
        sql.push_str(&format!(" limit {limit} offset {offset}"));

        (
            sqlx::query_as::<_, UserInfo>(&sql)
                .fetch_all(&*POOL)
                .await
                .unwrap_or_default(),
            sqlx::query(&count_sql)
                .fetch_one(&*POOL)
                .await
                .and_then(|row| row.try_get::<i64, _>("count"))
                .unwrap_or_default(),
        )
    }
}
