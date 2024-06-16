mod database;
mod keywords;
mod article;

use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;
use tokio::net::TcpListener;
use crate::article::Article;
use crate::database::pg_worker::connection;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/news", get(get_news));

    let listener = TcpListener::bind("localhost:80").await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();

    database::config::write().expect("Couldn't create config file!");
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

async fn get_news() -> impl IntoResponse {
    let mut articles: Vec<Article> = vec![];
    let rows = connection().await.unwrap().query("SELECT id, title, status, published_at, created_at, updated_at, tags, description, full_text FROM articles", &[]).await.unwrap();
    for row in rows {
        articles.push(Article {
            id: row.get(0),
            title: row.get(1),
            published_at: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
            description: row.get(5),
            full_text: row.get(6),
        });
    }

    Json(articles)
}
