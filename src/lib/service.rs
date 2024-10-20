// src/lib/service.rs

// dependencies
use axum::{extract::Request, Router, ServiceExt};
use shuttle_runtime::{Error, Service};
use std::net::SocketAddr;

// struct type to represent the server service
pub struct ShuttleTemplateAxum {
    pub app_router: Router,
}

// implement the Shuttle Service trait ont he NasaImageryViewerService type
#[shuttle_runtime::async_trait]
impl Service for ShuttleTemplateAxum {
    async fn bind(self, addr: SocketAddr) -> Result<(), Error> {
        let router = self.app_router;

        axum::serve(
            tokio::net::TcpListener::bind(addr).await?,
            ServiceExt::<Request>::into_make_service(router),
        )
        .await?;

        Ok(())
    }
}
