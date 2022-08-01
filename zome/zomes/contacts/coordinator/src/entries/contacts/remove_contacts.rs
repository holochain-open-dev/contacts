use hdk::prelude::*;

use contacts_integrity::*;
use contacts_integrity_types::*;

use crate::{
    helpers::check_latest_state,
    utils::error
};

pub fn remove_contacts_handler(
    agent_ids: Vec<AgentPubKey>,
) -> ExternResult<Vec<AgentPubKey>> {
    check_latest_state(&agent_ids, ContactType::Remove)?;

    let removed_contact = Contact::new(sys_time()?, agent_ids.clone(), ContactType::Remove, None);
    // create_entry(&removed_contact)?;
    // Ok(agent_ids)
    match create_entry(&EntryTypes::Contact(removed_contact.clone())) {
        Ok(_) => Ok(agent_ids),
        Err(_) => error("problems were encountered during creation of entry"),
    }
}
