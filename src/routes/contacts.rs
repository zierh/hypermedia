use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::Query, http::StatusCode, response::Html, routing::get, Router};
use hypermedia::{data::Contact, persistence::load_contacts};
use serde::Deserialize;

use crate::templates::ContactsView;

#[derive(Deserialize, Debug, Clone)]
pub struct Search {
    q: String,
}

pub fn get_route() -> Router {
    Router::new().route("/contacts", get(contact_list))
}

#[allow(unused_assignments)]
pub async fn contact_list(search_term: Option<Query<Search>>) -> impl IntoResponse {
    let mut result: Vec<Contact> = vec![];

    let search_option = &search_term.clone().map(|s| s.q.clone());
    let app_data = load_contacts();

    if let Some(search_term) = &search_term {
        // search in contacts
        let q = &search_term.q.to_lowercase();
        result = app_data
            .into_iter()
            .filter(|c| c.first.to_lowercase().contains(q) || c.last.to_lowercase().contains(q))
            .collect();
        println!("Results: {:?}", &result)
    } else {
        result = app_data;
    }

    let template = ContactsView {
        list: result,
        q: search_option.as_deref(),
    };
    let reply = template.render().unwrap();
    (StatusCode::OK, Html(reply).into_response())
}
