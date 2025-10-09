// file: api/index.rs (FINAL, COMPILER-FIXED & COMPLETE VERSION)

// --- IMPORTS (THE MISSING PIECE) ---
// Core web framework for routing and handlers
use axum::{
    routing::{get, post},
    Json, Router,
};
// For handling JSON data cleanly and safely
use serde_json::{json, Value};
// The tower library provides the `ServiceExt` trait, which gives us the `.oneshot()` method
use tower::{Service, ServiceExt};
// The core Vercel runtime for Rust and its necessary types
use vercel_runtime::{run, Body, Error, Response};
// The standard library's http module for working with request types
use http::Request;

// --- MAIN FUNCTION ---
// This is the entry point for the Vercel serverless function.
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

// --- MASTER HANDLER ---
// This function is called for EVERY incoming request.
pub async fn handler(req: Request<Body>) -> Result<Response<Body>, Error> {
    // We build our router here. It maps paths to our endpoint functions.
    let app = Router::new()
        .route("/", get(get_root))
        .route("/", post(discover_tools))
        .route("/api/invoke/scraper", post(invoke_isabella_scraper))
        .route("/api/query-datavault", post(query_datavault));

    // The .oneshot() method from the ServiceExt trait allows us to call our Axum router
    // as if it were a single function.
    let response = app.oneshot(req).await.unwrap();

    // Convert the Axum response back into a Vercel-compatible response.
    let (parts, body) = response.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    let vercel_response = Response::from_parts(parts, Body::from(body_bytes.to_vec()));
    
    Ok(vercel_response)
}


// --- API ENDPOINTS ---

// 1. The "Lobby"
async fn get_root() -> Json<Value> {
    Json(json!({"status": "Aegis Logistics Backend is online via vercel-rust."}))
}

// 2. The "Reception Desk"
async fn discover_tools() -> Json<Value> {
    let schema = json!({
        "openapi": "3.1.0",
        "info": {"title": "Aegis Logistics Tools", "version": "1.0"},
        "paths": {
            "/api/invoke/scraper": {
                "post": { "summary": "Isabella Web Scraper" }
            },
            "/api/query-datavault": {
                "post": { "summary": "Query Sovereign DataVault" }
            }
        }
    });
    Json(schema)
}

// 3. The Real Tools (Placeholders for now)
async fn invoke_isabella_scraper() -> Json<Value> {
    Json(json!({"enhanced_content": "This is the beautifully rewritten content from the scraper."}))
}

async fn query_datavault() -> Json<Value> {
    let simulated_results = json!([
        { "id": "tennyson-trail", "score": 0.95, "metadata": { "title": "Along the Tennyson Trail", "summary": "A sublime ribbon of nature..." }},
    ]);
    Json(json!({"status": "success", "results": simulated_results}))
}