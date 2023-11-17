use std::sync::{Arc, Mutex};

#[cfg(feature = "axum")]
pub mod axum;
pub mod storage;
pub mod tower;

pub trait SessionData: Clone + Send {
    const COOKIE_NAME: &'static str;
}

#[derive(Debug, Clone)]
pub struct Session<D: SessionData> {
    data: Arc<Mutex<Option<D>>>,
}

impl<D: SessionData> Session<D> {
    pub fn get(&mut self) -> Option<D> {
        (*self.data).lock().unwrap().clone()
    }
    pub fn set(&mut self, data: D) {
        self.data.lock().unwrap().replace(data);
    }
    pub fn clear(&mut self) {
        self.data.lock().unwrap().take();
    }
}

impl<D: SessionData> Default for Session<D> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
