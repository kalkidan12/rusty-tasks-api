use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    http::request::Parts,
};
use headers::{Authorization, authorization::Bearer};
use crate::{auth::jwt, error::AppError};
use uuid::Uuid;

pub struct AuthUser {
    pub user_id: Uuid,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, &())
                .await
                .map_err(|_| AppError::Unauthorized)?;

        let claims = jwt::verify_token(bearer.token());

        Ok(AuthUser {
            user_id: claims.sub,
            role: claims.role,
        })
    }
}
