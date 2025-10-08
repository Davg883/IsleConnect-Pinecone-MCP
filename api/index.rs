// file: api/index.rs (FINAL, CORRECTED VERSION)

use axum::{
    routing::{get, post},
    Router, Json,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower::ServiceExt;
use vercel_runtime::{run, Body, Error, Request, Response, LambdaRequest};

// The main entry point for the Vercel runtime
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

// The handler function that now correctly bridges Vercel and Axum
pub async fn handler(req: LambdaRequest) -> Result<Response<Body>, Error> {
    // We build the router inside the handler, which is the standard pattern.
    let app = Router::new()
        .route("/", get(get_root)) // The "Lobby"
        .route("/", post(discover_tools)) // The "Reception Desk"
        .route("/api/invoke/scraper", post(invoke_isabella_scraper))
        .route("/api/query-datavault", post(query_datavault));

    // Convert the Vercel LambdaRequest into a standard http::Request that Axum can handle.
    let (parts, body) = req.into_parts();
    let axum_body = axum::body::Body::from(body);
    let request = http::Request::from_parts(parts, axum_body);

    // Call our Axum router as a service
    let response = app.oneshot(request).await.unwrap();

    // Convert the Axum response back into a Vercel Response
    Ok(response.into())
}


// --- API Endpoints (These can remain simple) ---

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
                "post": { "summary": "Isabella Web Scraper" /* ... */ }
            },
            "/api/query-datavault": {
                "post": { "summary": "Query Sovereign DataVault" /* ... */ }
            }
        }
    });
    Json(schema)
}

// 3. The Real Tools (Placeholders for now)
// In the final version, these would have the security logic and full implementations.

async fn invoke_isabella_scraper() -> Json<Value> {
    Json(json!({"enhanced_content": "This is the beautifully rewritten content."}))
}

async fn query_datavault() -> Json<Value> {
    Json(json!({"status": "success", "results": []}))
}