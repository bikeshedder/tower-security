use std::{collections::HashMap, sync::Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use crate::session::{backend::SessionBackend, errors::SessionError, SessionData};

pub struct MemorySessionBackend<S: SessionData> {
    sessions: Mutex<HashMap<Uuid, S>>,
}

impl<S: SessionData> Default for MemorySessionBackend<S> {
    fn default() -> Self {
        Self {
            sessions: Default::default(),
        }
    }
}

#[async_trait]
impl<S: SessionData> SessionBackend<S> for MemorySessionBackend<S> {
    async fn load(&self, id: Uuid) -> Result<S, SessionError> {
        self.sessions
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(SessionError::InvalidSessionId)
    }
    async fn save(&self, id: Uuid, data: S) -> Result<(), SessionError> {
        self.sessions.lock().unwrap().insert(id, data);
        Ok(())
    }
}
