use mongoose::{DateTime, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: String,
    pub server_id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            server_id: String::default(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for Server {}
