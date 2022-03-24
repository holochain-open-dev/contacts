use hdk::prelude::*;

use super::helpers::list_added_or_blocked;

use super::{ContactOutput, ContactType};

pub fn list_added_handler() -> ExternResult<Vec<ContactOutput>> {
    Ok(list_added_or_blocked(ContactType::Add)?)
}
