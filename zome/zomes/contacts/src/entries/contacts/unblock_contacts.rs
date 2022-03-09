use hdk::prelude::*;

use super::helpers::check_latest_state;

use super::{Contact, ContactType};

pub fn unblock_contacts_handler(agent_ids: Vec<AgentPubKey>) -> ExternResult<Vec<AgentPubKey>> {
    check_latest_state(&agent_ids, ContactType::Unblock)?;
    let unblocked_contact =
        Contact::new(sys_time()?, agent_ids.clone(), ContactType::Unblock, None);
    create_entry(&unblocked_contact)?;
    Ok(agent_ids)
}
