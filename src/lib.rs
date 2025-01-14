use tracing::info;
use tracing_subscriber;

mod route;
mod util;

pub async fn run() {
    tracing_subscriber::fmt().compact().init();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!(listener = ?listener, "connecting");
    let app = route::create_route();
    axum::serve(listener, app).await.unwrap();
}
