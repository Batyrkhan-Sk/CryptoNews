# Cryptocurrency News Aggregator

## Overview
The Cryptocurrency News Aggregator is a Rust-based web service that collects and displays the latest news articles related to various cryptocurrencies. Users can search for news by entering the name or symbol of a cryptocurrency, and the application retrieves recent articles from multiple sources.

## Features
- Search for news by cryptocurrency name or symbol.
- Fetch data from multiple APIs, including CryptQNews and CoinGecko.
- Display news articles in a structured format, including title, source, date, summary, and link.
- Handle errors and manage API rate limits.
- Simple web interface for user interaction.
- (Optional) Implement caching to reduce API calls.

## Technology Stack
- **Backend**: Rust
- **Frontend**: Basic HTML/CSS or Rust-based UI (e.g., Yew)
- **Data Sources**: CryptQNews API, CoinGecko API
- **Caching & Storage**: Redis, SQLite, PostgreSQL (optional)

## Demonstation of Work

1. Main page without login/registration
<img width="1440" alt="Screenshot 2025-04-07 at 13 13 23" src="https://github.com/user-attachments/assets/82dd1ec1-b0f0-40e3-b87b-3c3d013791b1" />

2. Registration page
<img width="1440" alt="Screenshot 2025-04-07 at 13 13 48" src="https://github.com/user-attachments/assets/05177e7e-b6c7-45d4-8acd-d71831fa1080" />

3. Login page
<img width="1440" alt="Screenshot 2025-04-07 at 13 13 57" src="https://github.com/user-attachments/assets/c10b8b6d-ff20-41fc-bae1-96bc729018af" />

4. Main page with auth and filled search (Ex. Bitcoin)
<img width="1440" alt="Screenshot 2025-04-07 at 13 14 14" src="https://github.com/user-attachments/assets/2143477c-08f1-4c9e-b617-8b1a6d25f5c5" />

5. Bitcoin results
<img width="1440" alt="Screenshot 2025-04-07 at 13 14 20" src="https://github.com/user-attachments/assets/b074d702-54af-40bc-98e5-a8f7d9902c3d" />


## Installation
1. Clone the repository:
   ```
   git clone https://github.com/yourusername/crypto-news-aggregator.git
   ```
2. Navigate to the project directory:
   ```
   cd crypto-news-aggregator
   ```
3. Build the project:
   ```
   cargo build
   ```

## Usage
1. Run the application:
   ```
   cargo run
   ```
2. Open your web browser and navigate to `http://localhost:8000`.
3. Enter a cryptocurrency name or symbol in the search bar to retrieve the latest news articles.

## Examples
- Searching for "Bitcoin" will display the latest news articles related to Bitcoin.
- Searching for "ETH" will show news articles related to Ethereum.
