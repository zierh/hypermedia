use askama_axum::IntoResponse;
use axum::{extract::Query, http::StatusCode, response::Html, routing::{get, Route}, Router};
use askama::Template;
use serde::Deserialize;

use crate::{contact::Contact, templates::ContactsView};

#[derive(Deserialize, Debug, Clone)]
pub struct Search {
    q: String,
}

pub fn get_route() -> Router {
    Router::new()
        .route("/contacts", get(contact_list))
}

pub async fn contact_list(search_term: Option<Query<Search>>) -> impl IntoResponse {
    // let Query(search_term) = search_term.unwrap();
    println!("search_term: {:?}", &search_term);

    let contact_0: Contact = Contact {
        id: "1".to_string(),
        first_name: "Hannes".to_string(),
        last_name: "Zierh".to_string(),
        mail: "hannes.ziereis@gmail.com".to_string(),
        phone: "01602834531".to_string(),
    };
    let contact_1 = Contact {
        id: "2".to_string(),
        first_name: "Markus".to_string(),
        last_name: "Söder".to_string(),
        mail: "gott.koenig69@doener.bayern".to_string(),
        phone: "123456789".to_string(),
    };
    let contacts = vec![contact_0, contact_1];

    #[allow(unused_assignments)]
    let mut result: Vec<Contact> = vec![];

    let search_option = &search_term.clone().map(|s| s.q.clone());

    if let Some(search_term) = &search_term {
        // search in contacts
        let q = &search_term.q.to_lowercase();
        result = contacts
            .into_iter()
            .filter(|c| {
                c.first_name.to_lowercase().contains(q) || c.last_name.to_lowercase().contains(q)
            })
            .collect();
        println!("Results: {:?}", &result)
    } else {
        result = contacts;
    }

    let template = ContactsView {
        list: result,
        q: search_option.as_deref(),
    };
    let reply = template.render().unwrap();
    (StatusCode::OK, Html(reply).into_response())
}

