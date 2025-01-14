use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Extension};

#[derive(Clone)]
pub struct MyReq(pub String);

pub async fn read_mid(Extension(msg): Extension<MyReq>) -> String {
    msg.0
}

pub async fn set_mid(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let msg = "hello from middleware".to_owned();
    let ext = req.extensions_mut();
    ext.insert(MyReq(msg));
    Ok(next.run(req).await)
}
