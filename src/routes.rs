use axum::{
    extract::{Query, State},
    response::Html,
    http::StatusCode,
};
use serde::Deserialize;
use crate::{AppState, api};
use crate::api::NewsItem;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

pub async fn homepage(State(state): State<AppState>) -> Html<String> {
    let top_searches = state.cache.get_top_searches().await;
    
    let html = format!(r#"
        <!DOCTYPE html>
        <html>
            <head>
            <title>Crypto News Search</title>
                <style>
                body {{ font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }}
                .nav-container {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 10px 20px;
                    background-color: #333;
                    color: white;
                    border-radius: 4px;
                    margin-bottom: 20px;
                }}
                .nav-title {{
                    font-size: 1.2em;
                    font-weight: bold;
                }}
                .nav-buttons {{
                    display: flex;
                    gap: 10px;
                }}
                .nav-button {{
                    padding: 8px 15px;
                    background-color: #4CAF50;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                    text-decoration: none;
                    font-size: 14px;
                }}
                .nav-button:hover {{ background-color: #45a049; }}
                .search-container {{ text-align: center; margin: 40px 0; }}
                input[type="text"] {{ 
                    width: 60%; 
                    padding: 10px; 
                    font-size: 16px; 
                    border: 2px solid #ddd; 
                    border-radius: 4px; 
                }}
                button {{ 
                    padding: 10px 20px; 
                    font-size: 16px; 
                    background-color: #4CAF50; 
                    color: white; 
                    border: none; 
                    border-radius: 4px; 
                    cursor: pointer; 
                }}
                button:hover {{ background-color: #45a049; }}
                .top-searches {{
                    margin-top: 20px;
                    padding: 20px;
                    background-color: #f9f9f9;
                    border-radius: 4px;
                }}
                .top-searches h2 {{ margin-top: 0; }}
                .search-item {{
                    margin: 10px 0;
                    padding: 10px;
                    background-color: white;
                    border-radius: 4px;
                    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
                }}
                .user-welcome {{
                    text-align: center;
                    margin: 20px 0;
                    padding: 20px;
                    background-color: #e8f5e9;
                    border-radius: 4px;
                    color: #2e7d32;
                }}
                .user-features {{
                    display: flex;
                    justify-content: space-around;
                    margin: 30px 0;
                }}
                .feature-card {{
                    background-color: white;
                    padding: 20px;
                    border-radius: 8px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    width: 30%;
                    text-align: center;
                }}
                .feature-icon {{
                    font-size: 2em;
                    margin-bottom: 10px;
                }}
                .feature-title {{
                    font-weight: bold;
                    margin-bottom: 10px;
                }}
                .feature-description {{
                    color: #666;
                    font-size: 0.9em;
                }}
                .search-suggestions {{
                    margin-top: 10px;
                    font-size: 0.9em;
                    color: #666;
                }}
                .suggestion-item {{
                    display: inline-block;
                    margin: 5px;
                    padding: 5px 10px;
                    background-color: #f0f0f0;
                    border-radius: 15px;
                    cursor: pointer;
                }}
                .suggestion-item:hover {{
                    background-color: #e0e0e0;
                }}
                </style>
            </head>
            <body>
            <div class="nav-container">
                <div class="nav-title">Crypto News</div>
                <div class="nav-buttons" id="authButtons">
                    <a href="/register" class="nav-button">Register</a>
                    <a href="/login" class="nav-button">Login</a>
                </div>
            </div>
            
            <div id="userWelcome" style="display: none;" class="user-welcome">
                <h2>Welcome, <span id="username">User</span>!</h2>
                <p>You're logged in and can access all features of Crypto News.</p>
            </div>
            
            <div id="userFeatures" style="display: none;" class="user-features">
                <div class="feature-card">
                    <div class="feature-icon">🔍</div>
                    <div class="feature-title">Advanced Search</div>
                    <div class="feature-description">Search by symbol, name, or any related term</div>
                </div>
                <div class="feature-card">
                    <div class="feature-icon">📊</div>
                    <div class="feature-title">Sentiment Analysis</div>
                    <div class="feature-description">Get sentiment analysis for each news item</div>
                </div>
                <div class="feature-card">
                    <div class="feature-icon">🔔</div>
                    <div class="feature-title">Real-time Updates</div>
                    <div class="feature-description">Receive real-time news updates</div>
                </div>
            </div>
            
            <div class="search-container">
                <h1>Crypto News Search</h1>
                <form id="searchForm" action="/search" method="get">
                    <input type="text" id="searchInput" name="q" placeholder="Enter cryptocurrency name, symbol, or any term..." required>
                    <button type="submit">Search</button>
                </form>
                <div class="search-suggestions">
                    Try searching for: 
                    <span class="suggestion-item" onclick="searchFor('bitcoin')">Bitcoin</span>
                    <span class="suggestion-item" onclick="searchFor('eth')">Ethereum</span>
                    <span class="suggestion-item" onclick="searchFor('defi')">DeFi</span>
                    <span class="suggestion-item" onclick="searchFor('nft')">NFT</span>
                    <span class="suggestion-item" onclick="searchFor('blockchain')">Blockchain</span>
                </div>
            </div>
            
            <div class="top-searches">
                <h2>Top Searches</h2>
                {}
            </div>
            
            <script>
                // Check authentication status
                const token = localStorage.getItem('token');
                const authButtons = document.getElementById('authButtons');
                const userWelcome = document.getElementById('userWelcome');
                const userFeatures = document.getElementById('userFeatures');
                
                if (token) {{
                    // User is logged in
                    authButtons.innerHTML = `
                        <span class="nav-button" style="background-color: #666;">Welcome!</span>
                        <button class="nav-button" onclick="logout()">Logout</button>
                    `;
                    
                    // Show user-specific content
                    userWelcome.style.display = 'block';
                    userFeatures.style.display = 'flex';
                    
                    // Try to get username from token
                    try {{
                        const tokenParts = token.split('.');
                        const payload = JSON.parse(atob(tokenParts[1]));
                        document.getElementById('username').textContent = payload.sub;
                    }} catch (e) {{
                        console.error('Error parsing token:', e);
                    }}
                }}
                
                function logout() {{
                    localStorage.removeItem('token');
                    window.location.reload();
                }}
                
                function searchFor(term) {{
                    document.getElementById('searchInput').value = term;
                    document.getElementById('searchForm').submit();
                }}
                
                // Add event listener for search form
                document.getElementById('searchForm').addEventListener('submit', function(e) {{
                    const searchTerm = document.getElementById('searchInput').value.trim();
                    if (searchTerm === '') {{
                        e.preventDefault();
                        alert('Please enter a search term');
                    }}
                }});
            </script>
            </body>
        </html>
    "#, 
    if top_searches.is_empty() {
        "<p>No searches yet</p>".to_string()
    } else {
        top_searches.iter()
            .map(|(term, count)| {
                format!(
                    r#"<div class="search-item">{} - {} searches</div>"#,
                    term, count
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    });

    Html(html)
}

pub async fn handle_search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Html<String>, StatusCode> {
    // Normalize the search query
    let search_term = query.q.trim().to_lowercase();
    
    // Check if the search term is empty
    if search_term.is_empty() {
        return Ok(Html(format_error_html("Please enter a search term")));
    }
    
    let cache_key = format!("news:{}", search_term);
    
    // Try to get from cache first
    if let Some(cached_html) = state.cache.get(&cache_key).await {
        state.cache.increment_search_count(&search_term).await;
        return Ok(Html(cached_html));
    }

    // If not in cache, fetch from API
    match api::fetch_news(&search_term).await {
        Ok(news) => {
            let html = format_news_html(&news, &search_term);
            state.cache.set(&cache_key, &html).await;
            state.cache.increment_search_count(&search_term).await;
            Ok(Html(html))
        }
        Err(e) => {
            // Log the error for debugging
            tracing::error!("Error fetching news: {:?}", e);
            Ok(Html(format_error_html(&format!("Error fetching news: {}", e))))
        }
    }
}

fn format_error_html(error: &str) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Crypto News Search - Error</title>
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 20px;
                    background-color: #f5f5f5;
                }}
                .search-container {{
                    background-color: white;
                    padding: 20px;
                    border-radius: 8px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    margin-bottom: 20px;
                }}
                .search-form {{
                    display: flex;
                    gap: 10px;
                }}
                input[type="text"] {{
                    flex: 1;
                    padding: 10px;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                    font-size: 16px;
                }}
                button {{
                    padding: 10px 20px;
                    background-color: #007bff;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                    font-size: 16px;
                }}
                button:hover {{
                    background-color: #0056b3;
                }}
                .error-container {{
                    background-color: white;
                    padding: 20px;
                    border-radius: 8px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    margin-bottom: 20px;
                }}
                .error-title {{
                    color: #721c24;
                    margin-top: 0;
                }}
                .error-message {{
                    color: #333;
                    margin-bottom: 20px;
                }}
                .error-details {{
                    background-color: #f8f9fa;
                    padding: 15px;
                    border-radius: 4px;
                    border-left: 4px solid #dc3545;
                    margin-bottom: 20px;
                }}
                .error-solution {{
                    background-color: #e2e3e5;
                    padding: 15px;
                    border-radius: 4px;
                    margin-bottom: 20px;
                }}
                .error-solution h3 {{
                    margin-top: 0;
                    color: #383d41;
                }}
                .error-solution ul {{
                    margin-bottom: 0;
                }}
            </style>
        </head>
        <body>
            <div class="search-container">
                <form class="search-form" action="/search" method="get">
                    <input type="text" name="q" placeholder="Search for cryptocurrency news (e.g., BTC, ETH, Bitcoin)">
                    <button type="submit">Search</button>
                </form>
            </div>
            <div class="error-container">
                <h2 class="error-title">Error Fetching News</h2>
                <p class="error-message">{}</p>
                <div class="error-details">
                    <p><strong>Possible causes:</strong></p>
                    <ul>
                        <li>Invalid or missing API keys in the .env file</li>
                        <li>API service is temporarily unavailable</li>
                        <li>Network connectivity issues</li>
                    </ul>
                </div>
                <div class="error-solution">
                    <h3>How to fix:</h3>
                    <ul>
                        <li>Check your API keys in the .env file</li>
                        <li>Make sure you have valid API keys from:
                            <ul>
                                <li><a href="https://newsdata.io/" target="_blank">NewsData.io</a></li>
                                <li><a href="https://cryptonews-api.com/" target="_blank">CryptoNews API</a></li>
                            </ul>
                        </li>
                        <li>Try searching for a different cryptocurrency</li>
                    </ul>
                </div>
            </div>
        </body>
        </html>
        "#,
        error
    )
}

#[axum::debug_handler]
pub async fn handle_search_post(
    State(state): State<AppState>,
    axum::extract::Form(query): axum::extract::Form<SearchQuery>,
) -> Result<Html<String>, StatusCode> {
    handle_search(State(state), Query(query)).await
}

pub async fn cache_stats(State(state): State<AppState>) -> Html<String> {
    let stats = state.cache.get_stats().await;
    
    let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Cache Statistics</title>
            <style>
                body {{ font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }}
                .nav-container {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 10px 20px;
                    background-color: #333;
                    color: white;
                    border-radius: 4px;
                    margin-bottom: 20px;
                }}
                .nav-title {{
                    font-size: 1.2em;
                    font-weight: bold;
                }}
                .nav-buttons {{
                    display: flex;
                    gap: 10px;
                }}
                .nav-button {{
                    padding: 8px 15px;
                    background-color: #4CAF50;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                    text-decoration: none;
                    font-size: 14px;
                }}
                .nav-button:hover {{ background-color: #45a049; }}
                .stats-container {{ 
                    background-color: #f9f9f9; 
                    padding: 20px; 
                    border-radius: 4px; 
                    margin-top: 20px; 
                }}
                .stat-item {{ 
                    margin: 10px 0; 
                    padding: 10px; 
                    background-color: white; 
                    border-radius: 4px; 
                    box-shadow: 0 1px 3px rgba(0,0,0,0.1); 
                }}
            </style>
        </head>
                            <body>
            <div class="nav-container">
                <div class="nav-title">Crypto News</div>
                <div class="nav-buttons">
                    <a href="/register" class="nav-button">Register</a>
                    <a href="/login" class="nav-button">Login</a>
                </div>
            </div>
            <h1>Cache Statistics</h1>
            <div class="stats-container">
                <div class="stat-item">
                    <strong>Total Keys:</strong> {}
                </div>
                <div class="stat-item">
                    <strong>Total Memory Used:</strong> {:.2} MB
                </div>
                <div class="stat-item">
                    <strong>Hit Rate:</strong> {:.2}%
                </div>
            </div>
            <br>
            <a href="/" style="color: #4CAF50; text-decoration: none;">Back to Homepage</a>
                            </body>
        </html>
    "#, 
        stats.total_keys,
        stats.memory_used as f64 / 1024.0 / 1024.0,
        stats.hit_rate * 100.0
    );

    Html(html)
}

fn format_news_html(news_items: &[NewsItem], query: &str) -> String {
    // Get the display name for the cryptocurrency
    let display_name = match query.to_lowercase().as_str() {
        "btc" | "bitcoin" => "BITCOIN",
        "eth" | "ethereum" | "ether" => "ETHEREUM",
        "xrp" | "ripple" => "RIPPLE",
        "ltc" | "litecoin" => "LITECOIN",
        "doge" | "dogecoin" => "DOGECOIN",
        "ada" | "cardano" => "CARDANO",
        "dot" | "polkadot" => "POLKADOT",
        "sol" | "solana" => "SOLANA",
        "link" | "chainlink" => "CHAINLINK",
        "uni" | "uniswap" => "UNISWAP",
        _ => {
            // Try to match by removing any whitespace
            let normalized = query.to_lowercase().replace("", "");
            match normalized.as_str() {
                "btc" | "bitcoin" => "BITCOIN",
                "eth" | "ethereum" | "ether" => "ETHEREUM",
                "xrp" | "ripple" => "RIPPLE",
                "ltc" | "litecoin" => "LITECOIN",
                "doge" | "dogecoin" => "DOGECOIN",
                "ada" | "cardano" => "CARDANO",
                "dot" | "polkadot" => "POLKADOT",
                "sol" | "solana" => "SOLANA",
                "link" | "chainlink" => "CHAINLINK",
                "uni" | "uniswap" => "UNISWAP",
                _ => query
            }
        }
    }.to_string();
    
    let current_price = get_crypto_price(query);
    
    let news_html = if news_items.is_empty() {
        r#"
        <div class="no-news">
            <h2>No news found</h2>
            <p>No news was found for your search. Try using different keywords or check your spelling.</p>
            <ul>
                <li>Try using the full name of the cryptocurrency (e.g., "Bitcoin" instead of "BTC")</li>
                <li>Check for spelling errors</li>
                <li>Try searching for a different cryptocurrency</li>
            </ul>
        </div>
        "#.to_string()
    } else {
        news_items.iter().map(|item| {
            let sentiment_class = match item.sentiment.as_str() {
                "Positive" => "sentiment-positive",
                "Negative" => "sentiment-negative",
                _ => "sentiment-neutral",
            };
            
            let formatted_date = item.published_at.format("%a, %d %b %Y %H:%M:%S %z").to_string();
            
            format!(
                r#"
                <div class="news-item">
                    <h3 class="news-title">
                        <span class="sentiment-indicator {}"></span>
                        <a href="{}" target="_blank">{}</a>
                    </h3>
                    <div class="news-meta">
                        <span class="news-source">{}</span>
                        <span class="news-date">{}</span>
                        <span class="news-api">Source: {}</span>
                    </div>
                    <p class="news-summary">{}</p>
                </div>
                "#,
                sentiment_class,
                item.url,
                item.title,
                item.source,
                formatted_date,
                item.api_source,
                item.summary
            )
        }).collect::<Vec<String>>().join("\n")
    };
    
    format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Crypto News Search - {}</title>
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 20px;
                    background-color: #f5f5f5;
                }}
                .search-container {{
                    background-color: white;
                    padding: 20px;
                    border-radius: 8px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    margin-bottom: 20px;
                }}
                .search-form {{
                    display: flex;
                    gap: 10px;
                }}
                input[type="text"] {{
                    flex: 1;
                    padding: 10px;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                    font-size: 16px;
                }}
                button {{
                    padding: 10px 20px;
                    background-color: #007bff;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                    font-size: 16px;
                }}
                button:hover {{
                    background-color: #0056b3;
                }}
                .crypto-data {{
                    background-color: white;
                    padding: 20px;
                    border-radius: 8px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    margin-bottom: 20px;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }}
                .crypto-symbol {{
                    font-size: 24px;
                    font-weight: bold;
                    color: #333;
                }}
                .crypto-price {{
                    font-size: 24px;
                    font-weight: bold;
                    color: #28a745;
                }}
                .news-container {{
                    background-color: white;
                    padding: 20px;
                    border-radius: 8px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                }}
                .news-item {{
                    padding: 15px;
                    border-bottom: 1px solid #eee;
                    margin-bottom: 15px;
                }}
                .news-item:last-child {{
                    border-bottom: none;
                    margin-bottom: 0;
                }}
                .news-title {{
                    margin-top: 0;
                    margin-bottom: 10px;
                    font-size: 18px;
                }}
                .news-title a {{
                    color: #007bff;
                    text-decoration: none;
                }}
                .news-title a:hover {{
                    text-decoration: underline;
                }}
                .news-meta {{
                    display: flex;
                    gap: 15px;
                    color: #666;
                    font-size: 14px;
                    margin-bottom: 10px;
                }}
                .news-summary {{
                    color: #333;
                    line-height: 1.5;
                }}
                .sentiment-indicator {{
                    display: inline-block;
                    width: 12px;
                    height: 12px;
                    border-radius: 50%;
                    margin-right: 8px;
                    vertical-align: middle;
                }}
                .sentiment-positive {{
                    background-color: #28a745;
                }}
                .sentiment-negative {{
                    background-color: #dc3545;
                }}
                .sentiment-neutral {{
                    background-color: #6c757d;
                }}
                .no-news {{
                    text-align: center;
                    padding: 30px;
                    color: #666;
                }}
                .no-news h2 {{
                    color: #333;
                    margin-bottom: 15px;
                }}
                .no-news ul {{
                    text-align: left;
                    max-width: 400px;
                    margin: 20px auto;
                }}
                .no-news li {{
                    margin-bottom: 8px;
                }}
                .back-link {{
                    display: inline-block;
                    margin-top: 20px;
                    color: #007bff;
                    text-decoration: none;
                }}
                .back-link:hover {{
                    text-decoration: underline;
                }}
            </style>
        </head>
        <body>
            <div class="search-container">
                <form class="search-form" action="/search" method="get">
                    <input type="text" name="q" placeholder="Search for cryptocurrency news (e.g., BTC, ETH, Bitcoin)" value="{}">
                    <button type="submit">Search</button>
                </form>
            </div>
            
            <div class="crypto-data">
                <div class="crypto-symbol">Crypto Data: {}</div>
                <div class="crypto-price">Current Price: {}</div>
            </div>
            
            <div class="news-container">
                <h2>Latest News</h2>
                {}
            </div>
            
            <a href="/" class="back-link">Back to Search</a>
        </body>
        </html>
        "#,
        display_name,
        query,
        display_name,
        current_price,
        news_html
    )
}

fn get_crypto_price(query: &str) -> String {
    // This is a placeholder function that returns a mock price
    // In a real application, you would fetch the actual price from a cryptocurrency API
    match query.to_lowercase().as_str() {
        "btc" | "bitcoin" => "$80,000.00".to_string(),
        "eth" | "ethereum" | "ether" => "$3,500.00".to_string(),
        "xrp" | "ripple" => "$0.52".to_string(),
        "ltc" | "litecoin" => "$68.45".to_string(),
        "doge" | "dogecoin" => "$0.12".to_string(),
        "ada" | "cardano" => "$0.45".to_string(),
        "dot" | "polkadot" => "$7.23".to_string(),
        "sol" | "solana" => "$98.76".to_string(),
        "link" | "chainlink" => "$15.34".to_string(),
        "uni" | "uniswap" => "$5.67".to_string(),
        _ => {
            // Try to match by removing any whitespace and converting to lowercase
            let normalized = query.to_lowercase().replace(" ", "");
            match normalized.as_str() {
                "btc" | "bitcoin" => "$80,000.00".to_string(),
                "eth" | "ethereum" | "ether" => "$3,500.00".to_string(),
                "xrp" | "ripple" => "$0.52".to_string(),
                "ltc" | "litecoin" => "$68.45".to_string(),
                "doge" | "dogecoin" => "$0.12".to_string(),
                "ada" | "cardano" => "$0.45".to_string(),
                "dot" | "polkadot" => "$7.23".to_string(),
                "sol" | "solana" => "$98.76".to_string(),
                "link" | "chainlink" => "$15.34".to_string(),
                "uni" | "uniswap" => "$5.67".to_string(),
                _ => "N/A".to_string(),
            }
        }
    }
}
