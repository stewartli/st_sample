use axum::extract::{FromRef, State};

#[derive(Clone, FromRef)]
pub struct ShareData {
    pub msg: String,
}

pub async fn share_data(State(msg): State<String>) -> String {
    msg
}
