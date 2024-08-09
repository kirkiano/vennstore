use axum::{
    Router,
    routing::get,
    response::Html,
};
use tower_http::services::ServeDir;


pub fn routes() -> Router {
    Router::new().route("/", get(index))
        .nest_service("/assets", ServeDir::new("src/assets"))
}


use crate::{TEMPLATE, template::Context};


pub async fn index() -> Html<String> {
    let c = Context::new()
        .bind("name", "goomba");

    TEMPLATE.render("index.html", &c)
        .map(Html)
        .unwrap()
}