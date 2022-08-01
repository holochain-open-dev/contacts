use hdk::prelude::*;

use contacts_integrity::*;
use contacts_integrity_types::*;
use contacts_coordinator_types::*;

use crate::{
    helpers::check_latest_state,
    utils::error,
};

pub fn remove_from_category_handler(io: CategoryIO) -> ExternResult<CategoryIO> {
    check_latest_state(&io.agents, ContactType::RemoveFromCategory)?;
    let removed_contact = Contact::new(
        sys_time()?,
        io.agents.clone(),
        ContactType::RemoveFromCategory,
        None,
    );
    // create_entry(&removed_contact)?;
    // Ok(io)
    match create_entry(&EntryTypes::Contact(removed_contact.clone())) {
        Ok(_) => Ok(io),
        Err(_) => error("problems were encountered during creation of entry"),
    }
}
