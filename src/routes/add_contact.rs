use std::fmt;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Redirect},
    routing::{get, post},
    Form, Router,
};
use email_address::EmailAddress;
use hypermedia::{data::Contact, persistence::save_contact};
use serde::Deserialize;
use uuid::Uuid;

use crate::templates::CreateContact;

#[derive(Deserialize, Clone, Debug)]
pub struct AddContact {
    pub first: Option<String>,
    pub last: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl AddContact {
    pub fn new() -> Self {
        AddContact {
            first: None,
            last: None,
            phone: None,
            email: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddContactErrors {
    pub first: Option<String>,
    pub last: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl AddContactErrors {
    pub fn new() -> Self {
        AddContactErrors {
            first: None,
            last: None,
            phone: None,
            email: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewContact(AddContact, AddContactErrors);

impl NewContact {
    pub fn new() -> Self {
        NewContact(AddContact::new(), AddContactErrors::new())
    }
}

// impl fmt::Display for EmailError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "invalid email")
//     }
// }

pub fn get_route() -> Router {
    Router::new()
        .route("/contacts/new", get(new_contact_page))
        .with_state(NewContact::new())
        .route("/contacts/new", post(new_contact))
}

async fn new_contact(new_contact: Form<AddContact>) -> impl IntoResponse {
    let contact = Contact {
        id: Uuid::new_v4(),
        first: new_contact.first.clone().unwrap_or_default(),
        last: new_contact.last.clone().unwrap_or_default(),
        phone: new_contact.phone.clone().unwrap_or_default(),
        email: new_contact.email.clone().unwrap_or_default(),
    };

    if !EmailAddress::is_valid(&contact.email) || &contact.phone == "1" {
        let r = new_contact_page(Some(State(NewContact(
            AddContact {
                first: Some(contact.first),
                last: Some(contact.last),
                phone: Some(contact.phone),
                email: Some(contact.email),
            },
            AddContactErrors {
                first: None,
                last: None,
                phone: None,
                email: Some("Invalid MAIL".to_string()),
            },
        ))))
        .await
        .into_response();

        return (StatusCode::UNPROCESSABLE_ENTITY, r);
    }

    if save_contact(&contact).is_ok() {
        // messages.success("Contact created");
        (
            StatusCode::SEE_OTHER,
            Redirect::to("/contacts").into_response(), // TODO figure out how to return 201 and redirect
        )
    } else {
        // messages.error("Internal server error");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            new_contact_page(None).await.into_response(),
        )
    }
}

async fn new_contact_page(state: Option<State<NewContact>>) -> impl IntoResponse {
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
    (StatusCode::OK, Html(reply))
}
