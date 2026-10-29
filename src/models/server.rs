use mongoose::{doc, types::MongooseError, DateTime, IndexModel, IndexOptions, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: String,
    pub server_id: String,
    pub server_total: u64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            server_id: String::default(),
            server_total: 0,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for Server {}

impl Server {
    pub async fn migrate() -> Result<Vec<String>, MongooseError> {
        let indexes = [IndexModel::builder()
            .keys(doc! { "server_id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build()];
        let result = Self::create_indexes(&indexes).await?;
        Ok(result.index_names)
    }

    pub async fn find_or_create(server_id: &str) -> Result<Self, MongooseError> {
        if let Ok(exists) = Self::read(doc! { "server_id": server_id }).await {
            return Ok(exists);
        }
        let new_server = Self {
            server_id: server_id.to_string(),
            ..Default::default()
        };
        new_server.save().await
    }

    pub async fn increment_server_total(server_id: &str) -> Result<Self, MongooseError> {
        if let Ok(exists) = Self::update(
            doc! { "server_id": server_id },
            doc! {
                "$inc": {
                    "server_total": 1
                }
            },
        )
        .await
        {
            return Ok(exists);
        }
        let new_server = Self {
            server_total: 1,
            server_id: server_id.to_string(),
            ..Default::default()
        };
        new_server.save().await
    }

    pub async fn beer_count(server_id: &str) -> Result<u64, MongooseError> {
        let server = Self::find_or_create(server_id).await?;
        Ok(server.server_total)
    }
}
