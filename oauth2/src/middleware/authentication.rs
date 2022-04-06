use crate::models::token::Claims;
use crate::SECRET;
use axum::async_trait;
use axum::extract::FromRequest;
use axum::extract::RequestParts;
use axum::extract::TypedHeader;
use axum::response::Redirect;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken as jwt;
use jsonwebtoken::Validation;

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = Redirect;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        TypedHeader::<Authorization<Bearer>>::from_request(req)
            .await
            .map_err(|_| Redirect::to("/login"))
            .and_then(|bearer| {
                let key = jwt::DecodingKey::from_secret(SECRET);
                jwt::decode::<Claims>(bearer.token(), &key, &Validation::default())
                    .and_then(|token| Ok(token.claims))
                    .map_err(|_| Redirect::to("/login"))
            })
    }
}
