use axum::response::Html;

pub async fn hello() -> Html<String> {
    Html(format!("<h1>This is home page</h1>"))
}
