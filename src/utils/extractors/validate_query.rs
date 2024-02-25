use axum::extract::Request;
use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Query},
};
use validator::Validate;

use crate::errors::Errors;

pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedQuery<T>
where
    S: Send + Sync,
    T: Validate,
    Query<T>: FromRequestParts<S>,
{
    type Rejection = Errors;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(data) = Query::<T>::from_request(req, state)
            .await
            .map_err(|_| Errors::QueryRejection)?;

        data.validate()?;
        Ok(ValidatedQuery(data))
    }
}
