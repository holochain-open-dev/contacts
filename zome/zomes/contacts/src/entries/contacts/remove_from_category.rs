use hdk::prelude::*;

use super::helpers::check_latest_state;

use super::{CategoryIO, Contact, ContactType};

pub fn remove_from_category_handler(io: CategoryIO) -> ExternResult<CategoryIO> {
    check_latest_state(&io.agents, ContactType::RemoveFromCategory)?;
    let removed_contact = Contact::new(
        sys_time()?,
        io.agents.clone(),
        ContactType::RemoveFromCategory,
        None,
    );
    create_entry(&removed_contact)?;
    Ok(io)
}
