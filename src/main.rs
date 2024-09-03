mod contact;
mod templates;

use askama::Template;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use contact::Contact;
use serde::Deserialize;
use std::net::SocketAddr;
use templates::ContactList;

#[derive(Deserialize, Debug)]
struct Search {
    q: String,
}

// impl Default for Search {
//     fn default() -> Self {
//         Self { q: String::new() }
//     }
// }

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/contacts") }))
        .route("/contacts", get(contact_list));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn contact_list(search_term: Option<Query<Search>>) -> impl IntoResponse {
    // let Query(search_term) = search_term.unwrap_or_default();
    println!("search_term: {:?}", &search_term);

    let contacts = vec![Contact {
        id: "idString".to_string(),
        first_name: "Hannes".to_string(),
        last_name: "Zierh".to_string(),
        mail: "hannes.ziereis@gmail.com".to_string(),
        phone: "01602834531".to_string(),
    }];

    let mut result: Vec<Contact> = vec![];

    if let Some(search_term) = search_term {
        println!("Search for: {:?}", &search_term);
        let q = &search_term.q;
        result = contacts
            .into_iter()
            .filter(|c| c.first_name.contains(q) || c.last_name.contains(q))
            .collect();
        println!("Result: {:?}", &result)
    } else {
        result = contacts;
    }

    let template = ContactList { list: result };
    let reply = template.render().unwrap();

    (StatusCode::OK, Html(reply).into_response())
}

// async fn handler() -> impl IntoResponse {
//     let template = HelloWorld { name: "Hannes" };
//     let reply = template.render().unwrap();
//     (StatusCode::OK, Html(reply).into_response())
// }
