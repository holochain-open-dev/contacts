use hdk::prelude::*;

use super::{CategoryIO, Contact, ContactType, EntryTypes};
use super::helpers::check_latest_state;
use crate::utils::error;

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
