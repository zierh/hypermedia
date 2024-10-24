use email_address::EmailAddress;
// use hypermedia::{data::Contact, persistence::save_contact};
use regex::Regex;
use serde::Deserialize;

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

    pub fn validate(&self) -> Result<(), AddContactErrors> {
        let mut e = AddContactErrors::new();
        let mut valid = true;

        if self.first.as_ref().is_none() || self.first.as_ref().unwrap().is_empty() {
            e.first = Some("Can not be empty".to_string());
            valid = false;
        }

        if self.last.as_ref().is_none() || self.last.as_ref().unwrap().is_empty() {
            e.last = Some("Can not be empty".to_string());
            valid = false;
        }

        if !EmailAddress::is_valid(self.email.as_ref().unwrap()) {
            e.email = Some("Enter a valid email address".to_string());
            valid = false;
        }

        if self.phone.is_none()
            || !Regex::new(r"^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$")
                .unwrap()
                .is_match(self.phone.as_ref().unwrap_or(&String::new()))
        {
            e.phone = Some("Enter a valid phone number".to_string());
            valid = false;
        }

        if valid {
            Ok(())
        } else {
            Err(e)
        }
    }
}

impl Default for AddContact {
    fn default() -> Self {
        Self::new()
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

impl Default for AddContactErrors {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewContact(pub AddContact, pub AddContactErrors);

impl NewContact {
    pub fn new() -> Self {
        NewContact(AddContact::new(), AddContactErrors::new())
    }

    pub fn with_state(contact: &AddContact, error: AddContactErrors) -> Self {
        NewContact(contact.clone(), error)
    }
}

impl Default for NewContact {
    fn default() -> Self {
        Self::new()
    }
}
