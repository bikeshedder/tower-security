use std::task::{Context, Poll};

use tower_layer::Layer;
use tower_service::Service;

use super::storage::{SessionStorage, SessionStorageExt};

#[derive(Clone)]
pub struct SessionLayer<D> {
    storage_ext: SessionStorageExt<D>,
}

impl<D> SessionLayer<D> {
    pub fn new(storage: impl SessionStorage<D> + 'static) -> Self {
        Self {
            storage_ext: SessionStorageExt::new(storage),
        }
    }
}

impl<S, D> Layer<S> for SessionLayer<D> {
    type Service = SessionService<S>;

    fn layer(&self, service: S) -> Self::Service {
        SessionService { inner: service }
    }
}

#[derive(Clone)]
// This service implements the Log behavior
pub struct SessionService<S> {
    inner: S,
}

impl<S, ReqBody> Service<http::Request<ReqBody>> for SessionService<S>
where
    S: Service<http::Request<ReqBody>> + Clone,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: http::Request<ReqBody>) -> Self::Future {
        //request.extensions().get::<SessionStorageExt<D>>

        // take the service that was ready
        // See https://docs.rs/tower/latest/tower/trait.Service.html#be-careful-when-cloning-inner-services
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        inner.call(request)
    }
}
