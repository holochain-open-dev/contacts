use hdk::prelude::*;

use contacts_integrity::*;
use contacts_integrity_types::*;
use contacts_coordinator_types::*;

use crate::{
    helpers::check_latest_state, 
    utils::error
};

pub fn add_to_category_handler(io: CategoryIO) -> ExternResult<CategoryIO> {
    check_latest_state(&io.agents, ContactType::AddToCategory)?;
    let added_contact = Contact::new(
        sys_time()?,
        io.agents.clone(),
        ContactType::AddToCategory,
        Some(CategoryWithId {
            name: io.name.clone(),
            id: io.id.clone(),
        }),
    );
    // create_entry(&added_contact)?;
    // Ok(io)
    match create_entry(&EntryTypes::Contact(added_contact.clone())) {
        Ok(_) => Ok(io),
        Err(_) => error("problems were encountered during creation of entry"),
    }
}
