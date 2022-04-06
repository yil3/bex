use std::collections::HashMap;

use common::models::page::Page;

use crate::{
    models::user::{User, UserInfo},
    repository::user,
};
use common::utils;

pub struct Service;

impl Service {
    pub async fn register(&self) {}

    pub async fn update(&self, _user: &User) {}

    pub async fn page(&self, params: &HashMap<String, String>) -> Page<UserInfo> {
        let (limit, offset) = utils::page::fetch_limit_offset_by_map(&params);
        let (list, total) = user::Dao.page(limit, offset, params).await;
        Page::build(list, total, limit, offset)
    }
}
