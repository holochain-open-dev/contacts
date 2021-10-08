use hdk::prelude::*;

use super::helpers::list_added_or_blocked;

use super::ContactType;

pub fn list_blocked_handler() -> ExternResult<Vec<AgentPubKey>> {
    Ok(list_added_or_blocked(ContactType::Block)?)
}
