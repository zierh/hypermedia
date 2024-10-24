mod routes;
mod templates;

use crate::routes::contacts;

use axum::{response::Redirect, routing::get, Router};
use routes::{add_contact, view_contact};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/contacts") }))
        .merge(contacts::get_route())
        .merge(add_contact::get_route())
        .merge(view_contact::get_route())
        .nest_service("/assets", ServeDir::new("assets"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
