use hdk::prelude::*;

use super::helpers::check_latest_state;

use super::{Contact, ContactType};

pub fn remove_contacts_handler(agent_ids: Vec<AgentPubKey>) -> ExternResult<Vec<AgentPubKey>> {
    check_latest_state(&agent_ids, ContactType::Remove)?;

    let removed_contact = Contact::new(sys_time()?, agent_ids.clone(), ContactType::Remove, None);
    create_entry(&removed_contact)?;
    Ok(agent_ids)
}
