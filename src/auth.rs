use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    extract::State,
    http::StatusCode,
    Json,
    response::Html,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use futures_util::{SinkExt, StreamExt};
use crate::{AppState, api};
use serde_json::json;
use chrono;

const JWT_SECRET: &[u8] = b"your-secret-key"; // In production, use environment variable

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsUpdate {
    pub coin: String,
    pub news: Vec<crate::api::NewsItem>,
}

fn create_token(username: &str) -> Result<String, String> {
    let claims = Claims {
        sub: username.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    ).map_err(|e| format!("Failed to create token: {}", e))
}

pub async fn login_page() -> Html<String> {
    let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Login - Crypto News</title>
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
                .form-container {{
                    max-width: 400px;
                    margin: 0 auto;
                    padding: 20px;
                    background-color: #f9f9f9;
                    border-radius: 4px;
                    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
                }}
                .form-group {{
                    margin-bottom: 15px;
                }}
                label {{
                    display: block;
                    margin-bottom: 5px;
                    font-weight: bold;
                }}
                input {{
                    width: 100%;
                    padding: 8px;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                    box-sizing: border-box;
                }}
                button {{
                    width: 100%;
                    padding: 10px;
                    background-color: #4CAF50;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                    font-size: 16px;
                }}
                button:hover {{ background-color: #45a049; }}
                .error-message {{
                    color: #e74c3c;
                    margin-top: 10px;
                    text-align: center;
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
            <div class="form-container">
                <h2>Login</h2>
                <form id="loginForm">
                    <div class="form-group">
                        <label for="username">Username</label>
                        <input type="text" id="username" name="username" required>
                    </div>
                    <div class="form-group">
                        <label for="password">Password</label>
                        <input type="password" id="password" name="password" required>
                    </div>
                    <button type="submit">Login</button>
                    <div id="errorMessage" class="error-message"></div>
                </form>
                <p style="text-align: center; margin-top: 15px;">
                    Don't have an account? <a href="/register" style="color: #4CAF50;">Register</a>
                </p>
            </div>
            <script>
                document.getElementById('loginForm').addEventListener('submit', async (e) => {{
                    e.preventDefault();
                    
                    const username = document.getElementById('username').value;
                    const password = document.getElementById('password').value;
                    
                    try {{
                        const response = await fetch('/login', {{
                            method: 'POST',
                            headers: {{
                                'Content-Type': 'application/json',
                            }},
                            body: JSON.stringify({{
                                username,
                                password
                            }})
                        }});
                        
                        const data = await response.json();
                        
                        if (response.ok) {{
                            // Store the token
                            localStorage.setItem('token', data.token);
                            window.location.href = '/';
                        }} else {{
                            document.getElementById('errorMessage').textContent = data.error || 'Login failed';
                        }}
                    }} catch (error) {{
                        document.getElementById('errorMessage').textContent = 'An error occurred during login';
                    }}
                }});
            </script>
        </body>
        </html>
    "#);

    Html(html)
}

pub async fn register_page() -> Html<String> {
    let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Register - Crypto News</title>
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
                .form-container {{
                    max-width: 400px;
                    margin: 0 auto;
                    padding: 20px;
                    background-color: #f9f9f9;
                    border-radius: 4px;
                    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
                }}
                .form-group {{
                    margin-bottom: 15px;
                }}
                label {{
                    display: block;
                    margin-bottom: 5px;
                    font-weight: bold;
                }}
                input {{
                    width: 100%;
                    padding: 8px;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                    box-sizing: border-box;
                }}
                button {{
                    width: 100%;
                    padding: 10px;
                    background-color: #4CAF50;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                    font-size: 16px;
                }}
                button:hover {{ background-color: #45a049; }}
                .error-message {{
                    color: #e74c3c;
                    margin-top: 10px;
                    text-align: center;
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
            <div class="form-container">
                <h2>Register</h2>
                <form id="registerForm">
                    <div class="form-group">
                        <label for="username">Username</label>
                        <input type="text" id="username" name="username" required>
                    </div>
                    <div class="form-group">
                        <label for="email">Email</label>
                        <input type="email" id="email" name="email" required>
                    </div>
                    <div class="form-group">
                        <label for="password">Password</label>
                        <input type="password" id="password" name="password" required>
                    </div>
                    <button type="submit">Register</button>
                    <div id="errorMessage" class="error-message"></div>
                </form>
                <p style="text-align: center; margin-top: 15px;">
                    Already have an account? <a href="/login" style="color: #4CAF50;">Login</a>
                </p>
            </div>
            <script>
                document.getElementById('registerForm').addEventListener('submit', async (e) => {{
                    e.preventDefault();
                    
                    const username = document.getElementById('username').value;
                    const email = document.getElementById('email').value;
                    const password = document.getElementById('password').value;
                    
                    try {{
                        const response = await fetch('/register', {{
                            method: 'POST',
                            headers: {{
                                'Content-Type': 'application/json',
                            }},
                            body: JSON.stringify({{
                                username,
                                email,
                                password
                            }})
                        }});
                        
                        const data = await response.json();
                        
                        if (response.ok) {{
                            window.location.href = '/login';
                        }} else {{
                            document.getElementById('errorMessage').textContent = data.error || 'Registration failed';
                        }}
                    }} catch (error) {{
                        document.getElementById('errorMessage').textContent = 'An error occurred during registration';
                    }}
                }});
            </script>
        </body>
        </html>
    "#);

    Html(html)
}

pub async fn handle_login(
    State(state): State<AppState>,
    Json(credentials): Json<LoginRequest>,
) -> impl IntoResponse {
    match state.db.verify_user(&credentials.username, &credentials.password).await {
        Ok(user) => {
            match create_token(&user.username) {
                Ok(token) => {
                    (StatusCode::OK, Json(json!({
                        "token": token,
                        "username": user.username,
                        "email": user.email
                    })))
                }
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to create token"}))
                )
            }
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"}))
        )
    }
}

pub async fn handle_register(
    State(state): State<AppState>,
    Json(register): Json<RegisterRequest>,
) -> impl IntoResponse {
    match state.db.create_user(&register.username, &register.email, &register.password).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "message": "Registration successful"
            }))
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": e
            }))
        )
    }
}

pub async fn handle_ws(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let (_tx, _rx) = broadcast::channel::<String>(100);

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(coin) = serde_json::from_str::<String>(&text) {
                    let cache_key = format!("news:{}", coin);
                    
                    // Try to get from cache first
                    if let Some(cached_html) = state.cache.get(&cache_key).await {
                        let _ = sender.send(Message::Text(cached_html)).await;
                        continue;
                    }

                    // If not in cache, fetch from API
                    match api::fetch_news(&coin).await {
                        Ok(news) => {
                            let html = format_news_html(&coin, &news);
                            let _ = state.cache.set(&cache_key, &html).await;
                            let _ = sender.send(Message::Text(html)).await;
                        }
                        Err(e) => {
                            tracing::error!("Error fetching news: {:?}", e);
                            let _ = sender.send(Message::Text("Error fetching news".to_string())).await;
                        }
                    }
                }
            }
        }
    });

    recv_task.await.unwrap();
}

pub fn format_news_html(coin: &str, news: &[api::NewsItem]) -> String {
    let news_items: String = news.iter().map(|item| {
        let sentiment_class = match item.sentiment.as_str() {
            "Positive" => "positive",
            "Negative" => "negative",
            _ => "neutral"
        };

        let sentiment_emoji = match item.sentiment.as_str() {
            "Positive" => "🟢",
            "Negative" => "🔴",
            _ => "⚪"
        };

        format!(r#"
            <div class="news-item">
                <h3>{} {}</h3>
                <div class="news-meta">
                    <span class="source">Source: {}</span>
                    <span class="date">Date: {}</span>
                    <span class="sentiment {}">Sentiment: {}</span>
                </div>
                <p class="summary">{}</p>
                <a href="{}" target="_blank" class="read-more">Read more</a>
            </div>
        "#,
            sentiment_emoji,
            item.title,
            item.source,
            item.published_at.format("%Y-%m-%d %H:%M").to_string(),
            sentiment_class,
            item.sentiment,
            item.summary,
            item.url
        )
    }).collect();

    format!(r#"
        <html>
            <head>
                <title>News for {}</title>
                <style>
                    body {{ font-family: sans-serif; padding: 2em; background: #f9f9f9; }}
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
                    .news-item {{ 
                        background: white; 
                        padding: 1.5em; 
                        margin: 1em 0; 
                        border-radius: 8px;
                        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    }}
                    .news-meta {{
                        color: #666;
                        font-size: 0.9em;
                        margin: 0.5em 0;
                    }}
                    .source, .date, .sentiment {{
                        margin-right: 1em;
                    }}
                    .sentiment.positive {{ color: #2ecc71; }}
                    .sentiment.negative {{ color: #e74c3c; }}
                    .sentiment.neutral {{ color: #7f8c8d; }}
                    .summary {{
                        margin: 1em 0;
                        line-height: 1.6;
                    }}
                    .read-more {{
                        display: inline-block;
                        padding: 0.5em 1em;
                        background: #3498db;
                        color: white;
                        text-decoration: none;
                        border-radius: 4px;
                        margin-top: 1em;
                    }}
                    .read-more:hover {{
                        background: #2980b9;
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
                <h1>News for {}</h1>
                <div class="news-items">
                    {}
                </div>
                <br>
                <a href='/' class="read-more">Back to homepage</a>
                <script>
                    // Check authentication status
                    const token = localStorage.getItem('token');
                    const authButtons = document.getElementById('authButtons');
                    
                    if (token) {{
                        authButtons.innerHTML = `
                            <span class="nav-button" style="background-color: #666;">Welcome!</span>
                            <button class="nav-button" onclick="logout()">Logout</button>
                        `;
                    }}
                    
                    function logout() {{
                        localStorage.removeItem('token');
                        window.location.reload();
                    }}
                </script>
            </body>
        </html>
    "#, coin, coin, news_items)
} 