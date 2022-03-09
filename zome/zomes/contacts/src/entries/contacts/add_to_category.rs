use hdk::prelude::*;

use super::helpers::check_latest_state;

use super::{CategoryIO, CategoryWithId, Contact, ContactType};

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
    create_entry(&added_contact)?;
    Ok(io)
}
