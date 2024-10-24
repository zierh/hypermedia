use askama::Template;
use hypermedia::data::Contact;

use crate::routes::add_contact_types::{AddContact, AddContactErrors};

mod filters {
    pub fn display_some<T>(value: &Option<T>) -> askama::Result<String>
    where
        T: std::fmt::Display,
    {
        Ok(match value {
            Some(value) => value.to_string(),
            None => String::new(),
        })
    }
}

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloWorld<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "base.html")]
pub struct Base<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "contacts.html")]
pub struct ContactsView<'a> {
    pub list: Vec<Contact>,
    pub q: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "new.html")]
pub struct CreateContact {
    pub new_contact: AddContact,
    pub errors: AddContactErrors,
}

#[derive(Template)]
#[template(path = "view.html")]
pub struct ViewContact {
    pub contact: Contact,
}
