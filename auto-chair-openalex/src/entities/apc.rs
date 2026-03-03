use chrono::{DateTime, Utc};
use reqwest::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleProcessingCharge {
  pub currency: str,
  pub provenance: str,
  pub value: i64,
  pub value_usd: i64,
}
