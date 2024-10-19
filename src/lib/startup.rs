// src/lib/startup.rs

// dependencies
use crate::routes::health_check;
use crate::telemetry::MakeRequestUuid;
use axum::{http::HeaderName, routing::get, Router};
use tower::layer::Layer;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    normalize_path::NormalizePathLayer,
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// function to build and return a configured application that serves out the Zola built public assets
pub fn build() -> Router {
    // define the tracing layer
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .include_headers(true)
                .level(Level::INFO),
        )
        .on_response(DefaultOnResponse::new().include_headers(true));
    
    // define a layer to handle CORS (Cross Origin Resource Sharing)
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any);

    // create public assets, wrap them in a trace layer
    let public_assets = ServiceBuilder::new()
        .layer(&trace_layer)
        .service(ServeDir::new("public"));

    // build the router and wrap it with CORS and the telemetry layers
    let x_request_id = HeaderName::from_static("x-request-id");
    let api_routes = Router::new()
        .route("/health_check", get(health_check))
        .layer(cors)
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    x_request_id.clone(),
                    MakeRequestUuid,
                ))
                .layer(trace_layer)
                .layer(PropagateRequestIdLayer::new(x_request_id)),
        );

    // wrap the API routes with Normalize Path Layer
    let api_router = NormalizePathLayer::trim_trailing_slash().layer(api_routes);

    // combine the api and asset routes to make the complete router
    Router::new()
        .nest_service("/api", api_router)
        .nest_service("/", public_assets)
}
