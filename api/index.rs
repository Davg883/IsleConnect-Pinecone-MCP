// file: api/index.rs (FINAL, COMPILER-FIXED & COMPLETE VERSION)

// --- IMPORTS ---
// Core web framework for routing and handlers
use axum::{
    routing::{get, post},
    Json, Router,
};
// For handling JSON data cleanly
use serde_json::{json, Value};
// The tower library provides the `ServiceExt` trait, which gives us the `.oneshot()` method
use tower::ServiceExt;
// The core Vercel runtime for Rust and its necessary types
use vercel_runtime::{run, Body, Error, Response};

// --- MAIN FUNCTION ---
// This is the entry point for the Vercel serverless function.
// Its only job is to start the runtime and pass control to our main `handler` function.
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

// --- MASTER HANDLER ---
// This function is called for EVERY incoming request to your Vercel deployment.
// It creates the Axum router and lets it handle the request.
pub async fn handler(req: http::Request<Body>) -> Result<Response<Body>, Error> {
    // We build our router here. It maps paths to our endpoint functions.
    let app = Router::new()
        // The "Lobby": Responds to the Agent Builder's initial GET ping.
        .route("/", get(get_root))
        // The "Reception Desk": Responds to the POST discovery request with our tool "brochure".
        .route("/", post(discover_tools))
        // The real, secure tool endpoints.
        .route("/api/invoke/scraper", post(invoke_isabella_scraper))
        .route("/api/query-datavault", post(query_datavault));

    // The .oneshot() method from the ServiceExt trait allows us to call our Axum router
    // as if it were a single function, which is perfect for the serverless environment.
    let response = app.oneshot(req).await.unwrap();

    // Convert the Axum response back into a Vercel-compatible response.
    Ok(response.into())
}

// --- API ENDPOINTS ---
// These are the individual functions that handle requests for specific paths.

// 1. The "Lobby"
// Responds to simple GET requests to the base URL to confirm the server is online.
async fn get_root() -> Json<Value> {
    Json(json!({"status": "Aegis Logistics Backend is online via vercel-rust."}))
}

// 2. The "Reception Desk"
// Responds to the Agent Builder's POST request to "/" by providing the OpenAPI schema
// that describes all the available tools.
async fn discover_tools() -> Json<Value> {
    let schema = json!({
        "openapi": "3.1.0",
        "info": {"title": "Aegis Logistics Tools", "version": "1.0"},
        "paths": {
            "/api/invoke/scraper": {
                "post": {
                    "summary": "Isabella Web Scraper",
                    "description": "Takes a URL, scrapes it, and enhances the content using the Isabella brand voice.",
                    "operationId": "invoke_isabella_scraper",
                    "requestBody": { "required": true, "content": { "application/json": { "schema": {
                        "type": "object", "properties": { "url": { "type": "string" } }
                    }}}}
                }
            },
            "/api/query-datavault": {
                "post": {
                    "summary": "Query Sovereign DataVault",
                    "description": "Searches the curated Pinecone database for information about the Isle of Wight.",
                    "operationId": "query_datavault",
                    "requestBody": { "required": true, "content": { "application/json": { "schema": {
                        "type": "object", "properties": { "query": { "type": "string" } }
                    }}}}
                }
            }
        }
    });
    Json(schema)
}

// --- The Real Tools (Placeholders) ---
// These functions will be secured by the Auth middleware in the final version.
// For now, they return simple placeholder data.

// 3. The Isabella Scraper Tool
async fn invoke_isabella_scraper() -> Json<Value> {
    // In the real version, you would add `_: Auth` and `State(state)` as parameters
    // and implement the full Scrape -> Enhance -> Structure pipeline.
    Json(json!({"enhanced_content": "This is the beautifully rewritten content from the scraper."}))
}

// 4. The Pinecone Query Tool
async fn query_datavault() -> Json<Value> {
    // In the real version, you would add `_: Auth` and `State(state)` as parameters
    // and implement the full Pinecone query logic.
    let simulated_results = json!([
        { "id": "tennyson-trail", "score": 0.95, "metadata": { "title": "Along the Tennyson Trail", "summary": "A sublime ribbon of nature..." }},
    ]);
    Json(json!({"status": "success", "results": simulated_results}))
}