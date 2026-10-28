use mongoose::{doc, types::MongooseError, DateTime, IndexModel, Model};
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

impl Server {
    pub async fn migrate() -> Result<Vec<String>, MongooseError> {
        let indexes = [IndexModel::builder().keys(doc! { "server_id": 1 }).build()];
        let result = Self::create_indexes(&indexes).await?;
        Ok(result.index_names)
    }
}
