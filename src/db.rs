use sqlx::sqlite::SqlitePool;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::fs;
use std::path::Path;
use sqlx::Row;
use serde::{Serialize, Deserialize};
use serde;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Ensure the data directory exists
        let data_dir = Path::new("data");
        if !data_dir.exists() {
            fs::create_dir_all(data_dir).expect("Failed to create data directory");
        }

        let db_path = data_dir.join("users.db");
        if !db_path.exists() {
            fs::File::create(&db_path).expect("Failed to create database file");
        }

        let database_url = format!("sqlite:{}", db_path.display());
        println!("Connecting to database at: {}", database_url);
        
        let pool = SqlitePool::connect(&database_url).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS news_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                source TEXT NOT NULL,
                published_at TIMESTAMP NOT NULL,
                summary TEXT NOT NULL,
                url TEXT NOT NULL,
                sentiment TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;
        
        Ok(Database { pool })
    }
    
    pub async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<(), String> {
        let password_hash = hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))?;
        
        sqlx::query(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;
        
        Ok(())
    }
    
    pub async fn verify_user(&self, username: &str, password: &str) -> Result<User, String> {
        let row = sqlx::query("SELECT id, username, email, password_hash FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        match row {
            Some(row) => {
                let id: i64 = row.try_get("id").map_err(|e| format!("Failed to get id: {}", e))?;
                let username: String = row.try_get("username").map_err(|e| format!("Failed to get username: {}", e))?;
                let email: String = row.try_get("email").map_err(|e| format!("Failed to get email: {}", e))?;
                let password_hash: String = row.try_get("password_hash").map_err(|e| format!("Failed to get password_hash: {}", e))?;

                if verify(password, &password_hash).map_err(|e| format!("Failed to verify password: {}", e))? {
                    Ok(User {
                        id,
                        username,
                        email,
                        password_hash,
                    })
                } else {
                    Err("Invalid password".to_string())
                }
            }
            None => Err("User not found".to_string()),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
} 