use tower_cookies::Cookies;
use uuid::Uuid;

use super::errors::SessionError;

#[derive(Clone, Copy)]
pub enum SessionId {
    Missing,
    Invalid,
    Valid(Uuid),
}

impl SessionId {
    pub fn from_cookies(cookies: &Cookies, cookie_name: &str) -> Result<Self, SessionError> {
        let Some(cookie) = cookies.get(cookie_name) else {
            return Ok(Self::Missing);
        };
        let Ok(session_id) = Uuid::parse_str(cookie.value()) else {
            return Ok(Self::Invalid);
        };
        Ok(Self::Valid(session_id))
    }
    pub fn is_valid(&self) -> bool {
        matches!(self, SessionId::Valid(_))
    }
    pub fn valid(&self) -> Option<Uuid> {
        if let SessionId::Valid(id) = *self {
            Some(id)
        } else {
            None
        }
    }
}
