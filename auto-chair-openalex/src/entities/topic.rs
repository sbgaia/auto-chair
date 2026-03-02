use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub display_name: String,
    pub count: Option<i64>,
    pub score: Option<f64>,
    pub subfield: TopicClassification,
    pub field: TopicClassification,
    pub domain: TopicClassification,
    pub description: Option<String>,
    pub ids: Option<TopicIds>,
    pub keywords: Option<Vec<String>>,
    pub updated_date: Option<DateTime<Utc>>,
    pub works_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicClassification {
    pub id: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicIds {
    pub openalex: Option<String>,
    pub wikipedia: Option<String>,
}
