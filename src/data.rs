use rkyv::{Archive, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Archive, Deserialize, Serialize)]
pub struct Contact {
    pub id: Uuid,
    pub first: String,
    pub last: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Archive, Deserialize, Serialize)]
pub struct AppData {
    pub contacts: Vec<Contact>,
}
