// file: api/index.rs

use axum::{
    routing::{get, post},
    Router, Json,
};
use serde_json::{json, Value};
use vercel_runtime::{run, Body, Error, Request, Response};

// The main entry point for the Vercel runtime.
// It simply passes control to our Axum handler.
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

// The Axum handler that contains all our application's logic.
async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // We build the router inside the handler. This is the standard pattern for vercel-rust.
    let app = Router::new()
        .route("/", get(get_root)) // The "Lobby"
        .route("/", post(discover_tools)) // The "Reception Desk"
        .route("/api/invoke/scraper", post(invoke_isabella_scraper))
        .route("/api/query-datavault", post(query_datavault));

    // This converts the incoming Vercel request into a format Axum can understand
    // and then calls the correct route handler.
    Ok(axum_aws_lambda::axum_to_lambda(app, req).await)
}


// --- API Endpoints ---

// 1. The "Lobby"
async fn get_root() -> Json<Value> {
    Json(json!({"status": "Aegis Logistics Backend is online via vercel-rust."}))
}

// 2. The "Reception Desk" - Advertises our available tools
async fn discover_tools() -> Json<Value> {
    let schema = json!({
        "openapi": "3.1.0",
        "info": {"title": "Aegis Logistics Tools", "version": "1.0"},
        "paths": {
            "/api/invoke/scraper": {
                "post": { "summary": "Isabella Web Scraper" /* ... rest of your schema ... */ }
            },
            "/api/query-datavault": {
                "post": { "summary": "Query Sovereign DataVault" /* ... rest of your schema ... */ }
            }
        }
    });
    Json(schema)
}

// 3. The Real, Secure Tools (Simplified for this example)
// In a real app, you would add your security logic and full tool implementation here.

async fn invoke_isabella_scraper() -> Json<Value> {
    Json(json!({"enhanced_content": "This is the beautifully rewritten content."}))
}

async fn query_datavault() -> Json<Value> {
    Json(json!({"status": "success", "results": []}))
}