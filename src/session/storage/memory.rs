use std::{collections::HashMap, sync::Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use crate::session::SessionData;

use super::SessionStorage;

pub struct MemorySessionStorage<S: SessionData> {
    sessions: Mutex<HashMap<Uuid, S>>,
}

impl<S: SessionData> Default for MemorySessionStorage<S> {
    fn default() -> Self {
        Self {
            sessions: Default::default(),
        }
    }
}

#[async_trait]
impl<S: SessionData> SessionStorage<S> for MemorySessionStorage<S> {
    async fn load(&self, id: Uuid) -> Option<S> {
        self.sessions.lock().unwrap().get(&id).cloned()
    }
    async fn save(&self, id: Uuid, data: S) {
        self.sessions.lock().unwrap().insert(id, data);
    }
}
