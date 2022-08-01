use hdk::prelude::*;
use holo_hash::AgentPubKeyB64;

use super::helpers::{b64_to_agent_pk, check_latest_state};
use super::{Contact, ContactType, EntryTypes};
use crate::utils::error;

pub fn block_contacts_handler(agent_ids: Vec<AgentPubKeyB64>) -> ExternResult<Vec<AgentPubKeyB64>> {
    let agent_ids_raw = b64_to_agent_pk(agent_ids.clone());
    let me = agent_info()?.agent_latest_pubkey;
    // return err right away if trying to block oneself
    if let true = agent_ids_raw.contains(&me) {
        return error("cannot block yourself");
    }

    check_latest_state(&agent_ids_raw, ContactType::Block)?;
    let blocked_contact = Contact::new(sys_time()?, agent_ids_raw, ContactType::Block, None);
    // create_entry(&blocked_contact)?;
    // Ok(agent_ids)
    match create_entry(&EntryTypes::Contact(blocked_contact.clone())) {
        Ok(_) => Ok(agent_ids),
        Err(_) => error("problems were encountered during creation of entry"),
    }
}
