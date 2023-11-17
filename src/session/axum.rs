use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::session::{Session, SessionData};

#[async_trait]
impl<D: SessionData + 'static, State> FromRequestParts<State> for Session<D> {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &State,
    ) -> Result<Self, Self::Rejection> {
        let Some(session) = parts.extensions.get::<Self>() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't extract session from request. Did you forget to add the SessionLayer?",
            ));
        };
        let session = session.clone();
        // FIXME add proper error handling
        let _ = session.load().await;
        Ok(session)
    }
}
