use anyhow::{Ok, Result};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

use sse_server::get_router;

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let addr = format!("0.0.0.0:{}", 6687);
    let app = get_router();
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on {}", addr);
    info!("SSE Server started at http://localhost:6687");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
