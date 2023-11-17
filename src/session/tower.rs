use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use tower_cookies::{Cookie, Cookies};
use tower_layer::Layer;
use tower_service::Service;
use uuid::Uuid;

use super::{
    backend::{SessionBackend, SessionBackendWrapper},
    utils::SessionId,
    Session, SessionData, SessionInner,
};

#[derive(Clone)]
pub struct SessionLayer<D: SessionData> {
    backend: SessionBackendWrapper<D>,
}

impl<D: SessionData> SessionLayer<D> {
    pub fn new(storage: impl SessionBackend<D> + 'static) -> Self {
        Self {
            backend: SessionBackendWrapper::new(storage),
        }
    }
}

impl<S, D: SessionData> Layer<S> for SessionLayer<D> {
    type Service = SessionService<S, D>;

    fn layer(&self, service: S) -> Self::Service {
        SessionService {
            inner: service,
            backend: self.backend.clone(),
        }
    }
}

#[derive(Clone)]
// This service implements the Log behavior
pub struct SessionService<S, D: SessionData> {
    inner: S,
    backend: SessionBackendWrapper<D>,
}

impl<S, ReqBody, D> Service<http::Request<ReqBody>> for SessionService<S, D>
where
    D: SessionData + 'static,
    S: Service<http::Request<ReqBody>> + Clone + Send + 'static,
    S::Future: Send,
    S::Response: Send,
    S::Error: Send,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: http::Request<ReqBody>) -> Self::Future {
        let backend = self.backend.clone();
        // https://docs.rs/tower/latest/tower/trait.Service.html#be-careful-when-cloning-inner-services
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let cookies = request.extensions()
                .get::<Cookies>()
                .cloned()
                .expect("Cookies missing in request. Did you forget to add the tower_cookies::CookieManagerLayer _after_ tower_security::SessionManager?");

            let session_id = SessionId::from_cookies(&cookies, D::COOKIE_NAME).unwrap(); // TODO
            let session = Session::<D>::new(session_id, backend);
            request.extensions_mut().insert(session.clone());

            let response = inner.call(request).await;

            let session_inner = session.take_inner();

            match session_inner {
                // Empty session data and a session cookie is set. Remove the cookie.
                SessionInner {
                    id: SessionId::Invalid | SessionId::Valid(_),
                    data: None,
                } => {
                    cookies.remove(Cookie::new(D::COOKIE_NAME, ""));
                }
                // No session cookie. No data. Nothing to be done.
                SessionInner {
                    id: SessionId::Missing,
                    data: None,
                } => {}
                // Some session data available.
                SessionInner { id, data } => {
                    if let Some(data) = data {
                        let id = id.valid().unwrap_or_else(|| {
                            // Set cookie if not already set.
                            let id = Uuid::new_v4();
                            cookies.add(Cookie::new(D::COOKIE_NAME, id.to_string()));
                            id
                        });
                        session.backend.save(id, data).await.unwrap();
                    }
                }
            }

            response
        })
    }
}
