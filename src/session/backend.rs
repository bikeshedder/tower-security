use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use uuid::Uuid;

use super::{errors::SessionError, SessionData};

#[async_trait]
pub trait SessionBackend<D: SessionData>: Sync + Send {
    async fn load(&self, id: Uuid) -> Result<D, SessionError>;
    async fn save(&self, id: Uuid, data: D) -> Result<(), SessionError>;
}

pub struct SessionBackendWrapper<D: SessionData>(Arc<dyn SessionBackend<D>>);

impl<D: SessionData> Clone for SessionBackendWrapper<D> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D: SessionData> SessionBackendWrapper<D> {
    pub fn new(storage: impl SessionBackend<D> + 'static) -> Self {
        Self(Arc::new(storage))
    }
    pub async fn ping(&self) {}
}

impl<D: SessionData> Deref for SessionBackendWrapper<D> {
    type Target = dyn SessionBackend<D>;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
