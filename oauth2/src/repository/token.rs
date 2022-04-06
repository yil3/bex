use std::collections::HashMap;

use crate::{models::token::Token, POOL};
use sqlx::{postgres::PgQueryResult, Row};

pub struct Dao;

impl Dao {
    pub async fn insert(&self, token: Token) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("
        insert into token
        (id, owner, appliction, organization, code, access_token, refresh_token, exp, scope, token_type, create_at)
        values
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, now())",
        token.id, 
        token.owner, 
        token.appliction,
        token.organization, 
        token.code, 
        token.access_token, 
        token.refresh_token, 
        token.exp, 
        token.scope, 
        token.token_type)
        .execute(&*POOL)
        .await
    }

    pub async fn update(&self, token: Token) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("
        update token set 
        code = coalesce($2, code),
        access_token = coalesce($3, access_token),
        refresh_token = coalesce($4, refresh_token),
        update_at = now()
        where id = $1",
        token.id,
        token.code,
        token.access_token,
        token.refresh_token)
        .execute(&*POOL)
        .await
    }

    pub async fn page(&self, limit: u32, offset: u32, params: &HashMap<String, String>) -> (Vec<Token>, i64) {
        let mut count_sql = String::from("select count(0) from token where 1=1 ");
        let mut sql = String::new();
        sql.push_str("select * from token where 1=1");
        sql.push_str(" where 1=1");

        if let Some(owner) = params.get("owner") {
            if !owner.is_empty() {
                let query = &format!(" and owner = '{owner}'");
                sql.push_str(query);
                count_sql.push_str(query);
            }
        }

        if let Some(access_token) = params.get("access_token") {
            if !access_token.is_empty() {
                let query = &format!(" and access_token = '{access_token}'");
                sql.push_str(query);
                count_sql.push_str(query);
            }
        }

        sql.push_str(" order by create_at desc");
        sql.push_str(&format!(" limit {limit} offset {offset}"));

        (
            sqlx::query_as::<_, Token>(&sql)
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

    pub async fn delete(&self) {}
}
