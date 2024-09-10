use askama_axum::IntoResponse;
use axum::{Form, Router};
use serde::Deserialize;

use crate::contact::Contact;

// #[derive(Deserialize)]
// struct AddContact {
//     contact: Contact,
// }

// pub fn get_route() -> Router {
//     Router::new()
//         .route("/contacts/new", new_contact)
// }

// async fn new_contact(new_contact: Form<AddContact>) -> impl IntoResponse {
//     todo!();
// }
