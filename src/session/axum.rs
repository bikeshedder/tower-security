use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::session::{storage::SessionStorageExt, Session, SessionData};

#[async_trait]
impl<D: SessionData + 'static, State> FromRequestParts<State> for Session<D> {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &State,
    ) -> Result<Self, Self::Rejection> {
        // Return the session from the request extensions if it
        // was already loaded.
        if let Some(session) = parts.extensions.get::<Self>() {
            return Ok(session.clone());
        }

        // Create a new session object otherwise.
        let session = Session::<D>::default();
        parts.extensions.insert(session.clone());

        // Get session id from cookie
        let Some(cookies) = parts.extensions.get::<Cookies>() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't extract cookies from request.",
            ));
        };
        let Some(cookie) = cookies.get(D::COOKIE_NAME) else {
            return Ok(session);
        };
        let Ok(session_id) = Uuid::parse_str(cookie.value()) else {
            return Ok(session);
        };
        let Some(storage) = parts.extensions.get::<SessionStorageExt<D>>() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't extract session storage from request.",
            ));
        };
        let Some(data) = storage.load(session_id).await else {
            return Ok(session);
        };
        session.data.lock().unwrap().replace(data);
        Ok(session)
    }
}
