use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsArticle {
    pub id: Uuid,
    pub title: String,
    pub source: String,
    pub published_at: DateTime<Utc>,
    pub summary: String,
    pub url: String,
    pub symbol: String,
    pub cached_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Clone)]
pub struct AppState {
    pub redis_client: deadpool_redis::Pool,
    pub db_pool: sqlx::PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsItem {
    pub title: String,
    pub source: String,
    pub published_at: DateTime<Utc>,
    pub summary: String,
    pub url: String,
    pub sentiment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsResponse {
    pub status: String,
    pub total_results: i32,
    pub articles: Vec<Article>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub published_at: DateTime<Utc>,
    pub source: Source,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}
