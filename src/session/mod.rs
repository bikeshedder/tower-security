use std::{fmt, ops::Deref, sync::Arc};

use tokio::sync::{Mutex, MutexGuard};

use self::{backend::SessionBackendWrapper, errors::SessionError, utils::SessionId};

#[cfg(feature = "axum")]
pub mod axum;
pub mod backend;
pub mod backends;
pub mod errors;
pub mod tower;
pub mod utils;

pub trait SessionData: Clone + Send {
    const COOKIE_NAME: &'static str;
}

#[derive(Clone)]
pub struct Session<D: SessionData> {
    inner: Arc<Mutex<SessionInner<D>>>,
    backend: SessionBackendWrapper<D>,
}

#[cfg(feature = "derive")]
pub use tower_security_derive::SessionData;

impl<D: SessionData + fmt::Debug> fmt::Debug for Session<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = &*self.inner.try_lock().unwrap();
        f.debug_struct("Session")
            .field("data", &inner.data)
            .finish()
    }
}

impl<D: SessionData> Session<D> {
    pub fn new(id: SessionId, backend: SessionBackendWrapper<D>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(SessionInner { id, data: None })),
            backend,
        }
    }
    pub fn take_inner(&self) -> SessionInner<D> {
        let mut inner = (*self.inner).try_lock().unwrap();
        SessionInner {
            id: inner.id,
            data: inner.data.take(),
        }
    }
    pub async fn load(&self) -> Result<(), SessionError> {
        let mut inner = self.inner.try_lock().unwrap();
        if let SessionId::Valid(id) = inner.id {
            if inner.data.is_none() {
                match self.backend.load(id).await {
                    Ok(data) => inner.data = Some(data),
                    Err(err) => {
                        inner.id = SessionId::Invalid;
                        return Err(err);
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct SessionInner<D: SessionData> {
    id: SessionId,
    data: Option<D>,
}

impl<D: SessionData> Session<D> {
    pub fn get(&self) -> SessionDataGuard<D> {
        SessionDataGuard(self.inner.try_lock().unwrap())
    }
    pub fn set(&self, data: D) {
        self.inner.try_lock().unwrap().data.replace(data);
    }
    pub fn clear(&self) {
        self.inner.try_lock().unwrap().data.take();
    }
}

pub struct SessionDataGuard<'a, D: SessionData>(MutexGuard<'a, SessionInner<D>>);

impl<'a, D: SessionData> Deref for SessionDataGuard<'a, D> {
    type Target = Option<D>;
    fn deref(&self) -> &Self::Target {
        &self.0.data
    }
}
