use axum::{debug_handler, Json};
use axum::{extract::Path, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JobDetail {
    jobdetail: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    jobfile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobExtra {
    message_client: String,
    message_server: String,
    path_id: String,
    detail: String,
}

#[debug_handler]
pub async fn job(
    Path(id): Path<i32>,
    Query(detail): Query<JobDetail>,
    Json(body): Json<Job>,
) -> Json<JobExtra> {
    Json(JobExtra {
        message_client: body.jobfile,
        message_server: "Hello from server".to_owned(),
        path_id: id.to_string(),
        detail: detail.jobdetail,
    })
}
