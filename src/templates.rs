use askama::Template;

use crate::contact::Contact;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloWorld<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "contacts.html")]
pub struct ContactList {
    pub list: Vec<Contact>,
}
