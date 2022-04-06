use axum::Json;
use common::{models::resp::R, utils::string::IsEmpty};

use axum::response::IntoResponse;

use crate::{models::user::User, service::oauth};

pub async fn login(user: Json<User>) -> impl IntoResponse {
    if user.account.is_empty() || user.passwd.is_empty() {
        return Json(R::fail("account or passwd is not empty"));
    };
    let account = user.account.as_ref().unwrap();
    let passwd = user.passwd.as_ref().unwrap();
    match oauth::Service.login(account, passwd).await {
        Ok(token) => Json(R::success(token)),
        Err(e) => Json(R::fail(e)),
    }
}
