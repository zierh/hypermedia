use std::{error::Error, fmt};

use askama_axum::IntoResponse;
use axum::{routing::get, Form, Router};
use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// pub enum AddContactErrors {
//     Required,
//     NotAPhoneNumber,
// }

// impl fmt::Display for AddContactErrors {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "asdf")
//     }
// }

#[derive(Deserialize)]
pub struct AddContact {
    pub first: Option<String>,
    pub last: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    // errors: AddContactErrors,
}

pub fn get_route() -> Router {
    Router::new().route("/contacts/new", get(new_contact))
}

async fn new_contact(_new_contact: Form<AddContact>) -> impl IntoResponse {
    // todo!();
}
