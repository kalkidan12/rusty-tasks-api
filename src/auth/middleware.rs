use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use headers::{Authorization, authorization::Bearer, HeaderMapExt};
use uuid::Uuid;

use crate::{auth::jwt, error::AppError};

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
        let auth = parts
            .headers
            .typed_get::<Authorization<Bearer>>()
            .ok_or(AppError::Unauthorized)?;

        let claims = jwt::verify_token(auth.token());

        Ok(AuthUser {
            user_id: claims.sub,
            role: claims.role,
        })
    }
}
