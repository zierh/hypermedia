use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
};

use rkyv::{
    deserialize,
    rancor::{Error, Failure},
};

use crate::data::{AppData, ArchivedAppData, Contact};

pub fn load_contacts() -> Vec<Contact> {
    let mut buffer = Vec::new();
    let app_data: AppData;
    if let Ok(mut file) = OpenOptions::new().read(true).open("./contacts.sr") {
        file.read_to_end(&mut buffer).unwrap();

        let archived = rkyv::access::<ArchivedAppData, Failure>(&buffer).unwrap();

        app_data = deserialize::<AppData, Error>(archived).unwrap();
    } else {
        println!("FAILED OPENING FILE");
        app_data = AppData {
            contacts: Vec::new(),
        };
    };

    app_data.contacts
}

pub fn save_contact(c: &Contact) -> Result<(), std::io::Error> {
    let mut contacts = load_contacts();
    contacts.push(c.clone());

    let bytes = rkyv::to_bytes::<Error>(&contacts).unwrap();

    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true) // overwrite contents
        .write(true)
        .open("./contacts.sr")
        .unwrap();

    file.write_all(&bytes)
}
