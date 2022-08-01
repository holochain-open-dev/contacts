use hdk::prelude::*;

use contacts_integrity_types::*;
use contacts_coordinator_types::*;

use crate::helpers::list_added_or_blocked;

pub fn list_added_handler() -> ExternResult<Vec<ContactOutput>> {
    Ok(list_added_or_blocked(ContactType::Add)?)
}
