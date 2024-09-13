use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
}
