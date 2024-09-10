mod contact;
mod templates;
mod routes;

use crate::routes::contacts;

use askama_axum::IntoResponse;
use axum::{
    http::StatusCode, response::{Html, Redirect}, routing::get, Router
};
use routes::contacts::contact_list;
use serde::Deserialize;
use templates::HelloWorld;
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use askama::Template;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/contacts") }))
        .merge(contacts::get_route())
        // .route("/contacts", get(contact_list))
        .route("/hi", get(handler))
        .nest_service("/assets", ServeDir::new("assets"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let template = HelloWorld { name: "Hannes" };
    let reply = template.render().unwrap();
    (StatusCode::OK, Html(reply).into_response())
}
