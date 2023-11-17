use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use uuid::Uuid;

#[cfg(feature = "session-storage-memory")]
pub mod memory;

#[async_trait]
pub trait SessionStorage<D>: Sync + Send {
    async fn load(&self, id: Uuid) -> Option<D>;
    async fn save(&self, id: Uuid, data: D);
}

#[derive(Clone)]
pub struct SessionStorageExt<D>(Arc<dyn SessionStorage<D>>);

impl<D> SessionStorageExt<D> {
    pub fn new(storage: impl SessionStorage<D> + 'static) -> Self {
        Self(Arc::new(storage))
    }
}

impl<D> Deref for SessionStorageExt<D> {
    type Target = dyn SessionStorage<D>;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
