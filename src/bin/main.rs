// src/main.rs

// dependencies
use shuttle_runtime::Error;
use shuttle_template_axum_lib::service::ShuttleTemplateAxum;
use shuttle_template_axum_lib::{build, get_subscriber, init_subscriber};

// main function; configures tracing, builds the app router, starts the service
#[shuttle_runtime::main]
async fn main() -> Result<ShuttleTemplateAxum, Error> {
    // initialize tracing
    let subscriber = get_subscriber(
        "shuttle-template-vite-vanilla".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // build the router
    let app_router = build();

    // start the service
    Ok(ShuttleTemplateAxum { app_router })
}
