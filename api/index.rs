// file: api/index.rs (FINAL, COMPILER-FIXED VERSION)

use axum::{
    routing::{get, post},
    Router, Json,
};
use serde_json::{json, Value};
// REMOVED: No longer need SocketAddr
// use std::net::SocketAddr;
use tower::{Service, ServiceExt}; // CHANGED: Added `Service` and corrected the path for `ServiceExt`
use vercel_runtime::{run, Body, Error, Response}; // CHANGED: Removed unused imports

// The main entry point for the Vercel runtime
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

// The handler function that now correctly bridges Vercel and Axum
// CHANGED: The input type is now `http::Request<Body>`
pub async fn handler(req: http::Request<Body>) -> Result<Response<Body>, Error> {
    // We build the router inside the handler.
    let mut app = Router::new()
        .route("/", get(get_root))
        .route("/", post(discover_tools))
        .route("/api/invoke/scraper", post(invoke_isabella_scraper))
        .route("/api/query-datavault", post(query_datavault))
        .service_extensions(); // This line is optional but good practice

    // Call our Axum router as a service using the .oneshot() method.
    // The request `req` is already in the correct format.
    let response = app.oneshot(req).await.unwrap();

    // The response is already in a compatible format.
    Ok(response.into())
}


// --- API Endpoints (These can remain simple) ---
// ... (The rest of your code for get_root, discover_tools, etc., remains exactly the same) ...

async fn get_root() -> Json<Value> {
    Json(json!({"status": "Aegis Logistics Backend is online via vercel-rust."}))
}

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

async fn invoke_isabella_scraper() -> Json<Value> {
    Json(json!({"enhanced_content": "This is the beautifully rewritten content."}))
}

async fn query_datavault() -> Json<Value> {
    Json(json!({"status": "success", "results": []}))
}