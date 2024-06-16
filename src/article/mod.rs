use serde::{Deserialize, Serialize};

mod category;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub published_at: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub description: String,
    pub full_text: String,
}
