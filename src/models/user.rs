use mongoose::{doc, types::MongooseError, DateTime, IndexModel, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: String,
    pub server_id: String,
    pub server_total: u64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            user_id: String::default(),
            server_id: String::default(),
            server_total: 1,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for User {}

impl User {
    pub async fn migrate() -> Result<Vec<String>, MongooseError> {
        let indexes = [
            IndexModel::builder().keys(doc! { "user_id": 1 }).build(),
            IndexModel::builder().keys(doc! { "server_id": 1 }).build(),
        ];
        let result = Self::create_indexes(&indexes).await?;
        Ok(result.index_names)
    }

    pub async fn increment_user_total(
        server_id: &str,
        user_id: &str,
    ) -> Result<Self, MongooseError> {
        if let Ok(exists) = Self::update(
            doc! { "server_id": server_id, "user_id": user_id },
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
        let new_user = Self {
            server_id: server_id.to_string(),
            user_id: user_id.to_string(),
            ..Default::default()
        };
        new_user.save().await
    }
}
