use hdk::prelude::holo_hash::AgentPubKey;
use hdk::prelude::*;

use contacts_integrity::*;
use contacts_integrity_types::*;

use crate::{
    helpers::check_latest_state, 
    utils::error
};

pub fn add_contacts_handler(agent_ids: Vec<AgentPubKey>) -> ExternResult<Vec<AgentPubKey>> {
    check_latest_state(&agent_ids, ContactType::Add)?;
    let added_contact = Contact::new(sys_time()?, agent_ids.clone(), ContactType::Add, None);
    // create_entry(&added_contact)?;
    // Ok(agent_ids)
    match create_entry(&EntryTypes::Contact(added_contact.clone())) {
        Ok(_) => Ok(agent_ids),
        Err(_) => error("problems were encountered during creation of entry"),
    }
}
