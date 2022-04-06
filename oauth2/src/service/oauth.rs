use crate::SECRET;
use crate::{
    models::token::{Claims, TokenWrap},
    repository::oauth,
};
use common::utils;
use jsonwebtoken as jwt;
use tracing::error;

pub struct Service;

impl Service {
    pub async fn login(&self, account: &String, passwd: &String) -> Result<TokenWrap, &str> {
        oauth::Dao
            .sign_in(account, passwd)
            .await
            .map_err(|e| {
                error!("{e}");
                "the user does not exist!"
            })
            .map(|user| {
                let exp = utils::date::timestamp() + 14 * 24 * 60 * 60;
                let claims = Claims { user, exp };
                let header = jwt::Header::default();
                let key = jwt::EncodingKey::from_secret(SECRET);
                let token = jwt::encode(&header, &claims, &key).unwrap();
                let mut token_wrap = TokenWrap::default();
                token_wrap.access_token = token;
                token_wrap.exp = exp;
                token_wrap
            })
    }
}
