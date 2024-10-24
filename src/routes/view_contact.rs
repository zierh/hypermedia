use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    body::Body,
    extract::Path,
    http::{header::LOCATION, Response, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use axum_macros::debug_handler;
use hypermedia::persistence::load_contacts;
use uuid::Uuid;

use crate::templates::ViewContact;

pub fn get_route() -> Router {
    Router::new().route("/contacts/:id", get(view_contact))
}

#[debug_handler]
async fn view_contact(Path(id): Path<Uuid>) -> impl IntoResponse {
    let contacts = load_contacts();

    match contacts.into_iter().find(|c| c.id.eq(&id)) {
        Some(c) => {
            let template = ViewContact { contact: c };
            let reply = template.render().unwrap();

            (StatusCode::OK, Html(reply).into_response())
        }
        None => {
            let mut response = Response::builder().body(Body::empty()).unwrap();

            let headers = response.headers_mut();
            headers.insert(LOCATION, "/".parse().unwrap());

            (StatusCode::NOT_FOUND, response)
        }
    }
}
