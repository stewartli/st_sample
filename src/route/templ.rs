use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct MyTemplate {}

pub async fn templ() -> MyTemplate {
    return MyTemplate {};
}
