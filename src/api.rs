use reqwest;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::env;
use tracing;
use serde_json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsItem {
    pub title: String,
    pub source: String,
    pub url: String,
    pub published_at: DateTime<Utc>,
    pub summary: String,
    pub sentiment: String,
    pub api_source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsDataResponse {
    pub status: String,
    pub results: Vec<NewsDataArticle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsDataArticle {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    #[serde(rename = "pubDate")]
    pub pub_date: DateTime<Utc>,
    #[serde(rename = "source_id")]
    pub source_id: String,
}

pub async fn fetch_news(query: &str) -> Result<Vec<NewsItem>, String> {
    match fetch_newsdata(query).await {
        Ok(news) => {
            tracing::info!("Successfully fetched {} news items from NewsData.io", news.len());
            if news.is_empty() {
                return Err("No news found for your search query.".to_string());
            }
            Ok(news)
        },
        Err(e) => {
            tracing::error!("Failed to fetch from NewsData.io: {}", e);
            Err(format!("Error fetching news: {}", e))
        }
    }
}

async fn fetch_newsdata(query: &str) -> Result<Vec<NewsItem>, String> {
    let api_key = match env::var("NEWSDATA_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Err("NEWSDATA_API_KEY environment variable is not set".to_string());
        }
    };
    
    let normalized_query = normalize_query(query);
    
    let url = format!(
        "https://newsdata.io/api/1/news?apikey={}&q={}&language=en&size=10&category=business,technology",
        api_key, normalized_query
    );
    
    tracing::info!("Fetching from NewsData.io with query: {}", normalized_query);
    
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.map_err(|e| e.to_string())?;
        
        // Special handling for invalid API key
        if status == 401 && text.contains("The provided API key is not valid") {
            return Err("Invalid NewsData.io API key. Please check your .env file.".to_string());
        }
        
        return Err(format!("NewsData.io API returned error status {}: {}", status, text));
    }
    
    let text = response.text().await.map_err(|e| e.to_string())?;
    parse_newsdata_response(&text)
}

fn parse_newsdata_response(text: &str) -> Result<Vec<NewsItem>, String> {
    let data: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    
    // Check for API error messages
    if let Some(status) = data.get("status").and_then(|s| s.as_str()) {
        if status == "error" {
            if let Some(message) = data.get("message").and_then(|m| m.as_str()) {
                return Err(format!("NewsData.io API error: {}", message));
            }
        }
    }
    
    let mut news_items = Vec::new();
    
    if let Some(results) = data.get("results").and_then(|r| r.as_array()) {
        for item in results {
            if let (Some(title), Some(link), Some(pub_date), Some(source_id)) = (
                item.get("title").and_then(|t| t.as_str()),
                item.get("link").and_then(|l| l.as_str()),
                item.get("pubDate").and_then(|d| d.as_str()),
                item.get("source_id").and_then(|s| s.as_str()),
            ) {
                let description = item.get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or("");
                
                // Try parsing with different date formats and convert to UTC
                let published_at = if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(pub_date) {
                    dt.with_timezone(&chrono::Utc)
                } else if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(pub_date) {
                    dt.with_timezone(&chrono::Utc)
                } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(pub_date, "%Y-%m-%d %H:%M:%S") {
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
                } else {
                    return Err(format!("Failed to parse date: {}", pub_date));
                };
                
                let sentiment = analyze_sentiment(description);
                
                news_items.push(NewsItem {
                    title: title.to_string(),
                    url: link.to_string(),
                    source: source_id.to_string(),
                    published_at,
                    summary: description.to_string(),
                    sentiment,
                    api_source: "NewsData.io".to_string(),
                });
            }
        }
    }
    
    // Sort by date (newest first)
    news_items.sort_by(|a, b| b.published_at.cmp(&a.published_at));
    
    tracing::info!("Found {} news items from NewsData.io", news_items.len());
    Ok(news_items)
}

fn normalize_query(query: &str) -> String {
    let query = query.trim().to_lowercase();
    
    // Map common abbreviations to full names
    let query = match query.as_str() {
        "btc" => "bitcoin",
        "eth" | "ether" => "ethereum",
        "xrp" => "ripple",
        "ltc" => "litecoin",
        "doge" => "dogecoin",
        "ada" => "cardano",
        "dot" => "polkadot",
        "sol" => "solana",
        "link" => "chainlink",
        "uni" => "uniswap",
        _ => {
            // Try to match by removing any whitespace
            let normalized = query.replace(" ", "");
            match normalized.as_str() {
                "btc" => "bitcoin",
                "eth" | "ether" => "ethereum",
                "xrp" => "ripple",
                "ltc" => "litecoin",
                "doge" => "dogecoin",
                "ada" => "cardano",
                "dot" => "polkadot",
                "sol" => "solana",
                "link" => "chainlink",
                "uni" => "uniswap",
                _ => &query,
            }
        }
    };
    
    // Add "cryptocurrency" to the query to improve results
    format!("{} cryptocurrency", query)
}

fn analyze_sentiment(text: &str) -> String {
    // Simple sentiment analysis based on keyword matching
    let text = text.to_lowercase();
    let positive_words = ["bullish", "surge", "gain", "rise", "growth", "positive", "up", "high"];
    let negative_words = ["bearish", "crash", "drop", "fall", "decline", "negative", "down", "low"];

    let positive_count = positive_words.iter().filter(|word| text.contains(*word)).count();
    let negative_count = negative_words.iter().filter(|word| text.contains(*word)).count();

    if positive_count > negative_count {
        "Positive".to_string()
    } else if negative_count > positive_count {
        "Negative".to_string()
    } else {
        "Neutral".to_string()
    }
}

