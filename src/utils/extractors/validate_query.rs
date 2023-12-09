use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Query},
    http,
};
use validator::Validate;

use crate::errors::Errors;

pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedQuery<T>
where
    S: Send + Sync,
    B: Send + 'static,
    T: Validate,
    Query<T>: FromRequestParts<S>,
{
    type Rejection = Errors;

    async fn from_request(req: http::Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Query(data) =
            Query::<T>::from_request(req, state).await.map_err(|_| Errors::QueryRejection)?;

        data.validate()?;
        Ok(ValidatedQuery(data))
    }
}
