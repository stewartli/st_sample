use self::share_data::ShareData;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

mod hello;
mod job;
mod midware;
mod share_data;
mod templ;

pub fn create_route() -> Router {
    let shared_data = ShareData {
        msg: "hello from state".to_owned(),
    };
    Router::new()
        .route("/", get(hello::hello))
        .route("/job/:id", post(job::job))
        .route("/mid", get(midware::read_mid))
        .layer(middleware::from_fn(midware::set_mid))
        .route("/share", get(share_data::share_data))
        .with_state(shared_data)
        .route("/templ", get(templ::templ))
        .nest_service("/assets", ServeDir::new("asset"))
}
