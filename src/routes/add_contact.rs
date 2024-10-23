use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    body::Body,
    extract::State,
    http::{header::LOCATION, Response, StatusCode},
    response::Html,
    routing::{get, post},
    Form, Router,
};
use axum_macros::debug_handler;
use hypermedia::{
    data::Contact,
    persistence::save_contact,
    route_data::add_contact::{AddContact, AddContactErrors, NewContact},
};
use uuid::Uuid;

use crate::templates::CreateContact;

pub fn get_route() -> Router {
    Router::new()
        .route("/contacts/new", get(new_contact_page))
        .with_state(NewContact::new())
        .route("/contacts/new", post(new_contact))
}

#[debug_handler]
async fn new_contact(Form(new_contact): Form<AddContact>) -> impl IntoResponse {
    if let Err(e) = new_contact.validate() {
        let contact = Some(State(NewContact::with_state(&new_contact, e)));
        return (StatusCode::BAD_REQUEST, new_contact_page(contact).await);
    }

    let contact = Contact {
        id: Uuid::new_v4(),
        first: new_contact.first.clone().unwrap_or_default(),
        last: new_contact.last.clone().unwrap_or_default(),
        phone: new_contact.phone.clone().unwrap_or_default(),
        email: new_contact.email.clone().unwrap_or_default(),
    };

    if save_contact(&contact).is_ok() {
        // messages.success("Contact created");
        let mut response = Response::builder()
            // .status(StatusCode::ACCEPTED)
            .body(Body::empty())
            .unwrap();

        let headers = response.headers_mut();
        headers.insert(LOCATION, "/".parse().unwrap());

        (
            StatusCode::ACCEPTED,
            new_contact_page(Some(State(NewContact::new()))).await,
        )
    } else {
        // messages.error("Internal server error");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            new_contact_page(None).await,
        )
    }
}

async fn new_contact_page(state: Option<State<NewContact>>) -> Html<String> {
    // let messages = messages
    //     .into_iter()
    //     .map(|message| format!("{}: {}", message.level, message))
    //     .collect::<Vec<_>>()
    //     .join(", ");

    let template: CreateContact;
    if let Some(State(c)) = state {
        template = CreateContact {
            new_contact: AddContact {
                first: c.0.first,
                last: c.0.last,
                phone: c.0.phone,
                email: c.0.email,
            },
            errors: c.1,
        };
    } else {
        template = CreateContact {
            new_contact: AddContact {
                first: None,
                last: None,
                phone: None,
                email: None,
            },
            errors: AddContactErrors::new(),
        };
    }

    let reply = template.render().unwrap();
    // (StatusCode::OK, Html(reply))

    Html(reply)
}
